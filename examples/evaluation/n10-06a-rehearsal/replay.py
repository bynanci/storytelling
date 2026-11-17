#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Capture eight synthetic native CLI cases; creates no participant evidence."""
import argparse
import base64
import copy
from datetime import datetime, timezone
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'tests/fixtures/n10-06a-draft'
NAMES = ['seconds-unresolved.draft.json', 'frames-30-unresolved.draft.json',
         'frames-ntsc-unresolved.draft.json']


class EvidenceMismatch(ValueError):
    pass


def require(condition, reason):
    if not condition:
        raise EvidenceMismatch(reason)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encode(value):
    return (json.dumps(value, ensure_ascii=False, indent=2) + '\n').encode('utf-8')


def cases(sources):
    seconds, frames, ntsc = [json.loads(sources[name]) for name in NAMES]
    rows = []

    def add(case_id, value, code=None, path=None):
        rows.append((case_id, copy.deepcopy(value), code, path))

    add('decimal-25', seconds)
    value = copy.deepcopy(seconds)
    value['coverage']['mappings'][0]['source_range'].update(start='0', end='0.1')
    add('decimal-01', value)
    add('frames-30', frames)
    add('frames-ntsc', ntsc)
    value = copy.deepcopy(seconds)
    value['coverage']['mappings'][0]['source_range']['start'] = '2,5'
    add('decimal-comma', value, 'draft_time.invalid_decimal',
        '$.coverage.mappings[0].source_range.start')
    value = copy.deepcopy(seconds)
    value['coverage']['asset_catalog']['assets'][0]['duration']['numerator'] = '5'
    value['coverage']['mappings'][0]['source_range'].update(start='0', end='6')
    add('beyond-duration', value, 'video_coverage.invalid_range',
        '$.mappings[0].source_range.end')
    value = copy.deepcopy(frames)
    value['coverage']['mappings'][0]['source_range']['timing'] = 'variable'
    add('vfr-frames', value, 'draft_time.frame_timing_not_supported',
        '$.coverage.mappings[0].source_range.timing')
    value = copy.deepcopy(seconds)
    second = copy.deepcopy(value['coverage']['mappings'][0])
    second['unit_ref'] = value['coverage']['selected_unit_refs'][1]
    value['coverage']['mappings'][1] = second
    add('declaration-complete', value)
    return rows


def check_canonical_declarations(case_id, coverage):
    """Independent exact arithmetic for this fixed synthetic protocol."""
    spans = {
        'decimal-25': (Fraction('2.5'), Fraction('4.5')),
        'decimal-01': (Fraction('0'), Fraction('0.1')),
        'frames-30': (Fraction(75, 30), Fraction(135, 30)),
        'frames-ntsc': (Fraction(0), 150 * Fraction(1001, 30000)),
        'declaration-complete': (Fraction('2.5'), Fraction('4.5')),
    }
    start, end = spans[case_id]
    refs = [{'unit_id': 'S02', 'revision': 1}, {'unit_id': 'S03', 'revision': 1}]
    mapped = {'unit_ref': refs[0], 'status': 'mapped',
              'asset_ref': {'asset_id': 'CLIP-01', 'revision': 1},
              'source_range': {
                  'start': {'numerator': str(start.numerator), 'denominator': str(start.denominator)},
                  'end': {'numerator': str(end.numerator), 'denominator': str(end.denominator)}}}
    second = {'unit_ref': refs[1], 'status': 'unresolved',
              'reason': 'No video asset was selected for this unit.'}
    if case_id == 'declaration-complete':
        second = copy.deepcopy(mapped)
        second['unit_ref'] = refs[1]
    require(coverage['selected_unit_refs'] == refs and coverage['mappings'] == [mapped, second],
            'wrong_canonical_declarations')


def invoke(cli, args, directory, payload=None):
    start = time.perf_counter_ns()
    try:
        result = subprocess.run([str(cli), *args], input=payload,
                                stdin=subprocess.DEVNULL if payload is None else None,
                                cwd=directory, capture_output=True, timeout=10, check=False)
        code, stdout, stderr, timed_out = result.returncode, result.stdout, result.stderr, False
    except subprocess.TimeoutExpired as error:
        code, stdout, stderr, timed_out = None, error.stdout or b'', error.stderr or b'', True
    elapsed = time.perf_counter_ns() - start
    return {'arguments': args, 'exit_code': code, 'timed_out': timed_out,
            'process_elapsed_ns': elapsed, 'stdout_base64': base64.b64encode(stdout).decode('ascii'),
            'stderr_base64': base64.b64encode(stderr).decode('ascii'),
            'stdout_sha256': sha(stdout), 'stderr_sha256': sha(stderr)}, stdout, stderr


def replay(cli, source_commit):
    report = {'record_version': 'n10-a-rehearsal-1', 'evidence_kind': 'synthetic_machine_replay',
              'recorded_at_utc': datetime.now(timezone.utc).isoformat(), 'status': 'failed',
              'human_participants': 0, 'human_completion_rate': None, 'author_elapsed_seconds': None,
              'host_observations': 0, 'maintainer_acceptance': None,
              'source_commit_supplied': source_commit, 'source_build_binding_verified_by_tool': False,
              'script_sha256': sha(Path(__file__).read_bytes()), 'cases': [], 'cli_calls': 0,
              'planned_case_count': 8}
    try:
        binary = cli.read_bytes()
        report['binary_sha256'] = sha(binary)
        sources = {name: (FIXTURES / name).read_bytes() for name in NAMES}
        report['fixture_sha256'] = {name: sha(data) for name, data in sources.items()}
        for case_id, value, code, path in cases(sources):
            data = encode(value)
            row = {'case_id': case_id, 'input_utf8': data.decode('utf-8'), 'input_sha256': sha(data),
                   'expected_error_code': code, 'expected_error_path': path, 'calls': []}
            report['cases'].append(row)
            with tempfile.TemporaryDirectory(prefix='n10-synthetic-replay-') as tmp:
                source = Path(tmp) / 'input.json'
                source.write_bytes(data)

                def call(args, payload=None):
                    observation, stdout, stderr = invoke(cli, args, tmp, payload)
                    row['calls'].append(observation)
                    report['cli_calls'] += 1
                    require(not observation['timed_out'], 'native_cli_timeout')
                    require(not stderr, 'unexpected_stderr')
                    require(observation['exit_code'] == (1 if code else 0), 'unexpected_exit')
                    require(source.read_bytes() == data, 'input_changed')
                    return stdout

                original = call(['draft-video-coverage-input', 'input.json'])
                stdin = call(['draft-video-coverage-input', '-'], data)
                repeated = call(['draft-video-coverage-input', 'input.json'])
                require(original == stdin == repeated, 'nondeterministic_native_output')
                row['deterministic_file_stdin'] = True
                row['preview'] = json.loads(original)
                if code:
                    error = row['preview']['error']
                    require(error['code'] == code and error['path'] == path, 'wrong_error_identity')
                    require('coverage' not in row['preview'], 'invalid_input_emitted_coverage')
                    emitted = call(['draft-video-coverage-input', 'input.json', '--emit-v1'])
                    require(json.loads(emitted) == row['preview'], 'emit_error_changed')
                else:
                    require(row['preview']['scope'] == 'declarations_only', 'verification_scope_widened')
                    check_canonical_declarations(case_id, row['preview']['coverage'])
                    emitted = call(['draft-video-coverage-input', 'input.json', '--emit-v1'])
                    require(json.loads(emitted) == row['preview']['coverage'], 'emit_coverage_changed')
                    authority = call(['render-video-coverage', '-'], emitted)
                    require(json.loads(authority) == row['preview']['validation'], 'v1_result_changed')
                    row['accepted_v1_result'] = json.loads(authority)
                row['input_unchanged'] = source.read_bytes() == data
        report['binary_unchanged'] = sha(cli.read_bytes()) == report['binary_sha256']
        report['source_fixtures_unchanged'] = all((FIXTURES / n).read_bytes() == b for n, b in sources.items())
        require(report['binary_unchanged'] and report['source_fixtures_unchanged'], 'source_or_binary_changed')
        require(report['cli_calls'] == 37, 'unexpected_call_count')
        report['status'] = 'passed'
    except (OSError, ValueError, KeyError, TypeError) as error:
        report['failure'] = str(error) if isinstance(error, EvidenceMismatch) else 'capture_or_input_error'
    report['case_count'] = len(report['cases'])
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cli', type=Path, required=True)
    parser.add_argument('--source-commit', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if not re.fullmatch('[0-9a-f]{40}', args.source_commit):
        parser.error('source commit must be a full lowercase SHA; this is supplied metadata only')
    try:
        # Reserve once before invoking anything; old receipts cannot be silently replaced.
        with args.output.open('xb') as output:
            report = replay(args.cli.resolve(), args.source_commit)
            output.write(encode(report))
    except FileExistsError:
        print('output already exists; use a new receipt path', file=sys.stderr)
        return 2
    except OSError:
        print('output unavailable', file=sys.stderr)
        return 2
    print(json.dumps({'status': report['status'], 'cases_observed': len(report['cases']),
                      'cli_calls': report['cli_calls'], 'human_participants': 0}))
    return 0 if report['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
