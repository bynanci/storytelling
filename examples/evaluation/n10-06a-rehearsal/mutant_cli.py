#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Portable test-only native-output fault shim; not an executable sandbox."""
import base64
from fractions import Fraction
import importlib.util
import json
from pathlib import Path
import sys


def change(document, fault):
    if fault['kind'] == 'numeric_alias':
        value = document
        for key in fault['path'][:-1]:
            value = value[key]
        value[fault['path'][-1]] = fault['value']
        return
    coverage = document.get('coverage', document)
    copies = [coverage]
    if 'validation' in document:
        copies.append(document['validation'])
    for value in copies:
        mappings = value['mappings']
        if fault['kind'] == 'range':
            span = mappings[fault['mapping_index']]['source_range']
            endpoint = span[fault['endpoint']]
            changed = Fraction(int(endpoint['numerator']), int(endpoint['denominator'])) + Fraction(1, 1000)
            span[fault['endpoint']] = {'numerator': str(changed.numerator),
                                       'denominator': str(changed.denominator)}
        elif fault['kind'] == 'mapping_refs':
            mappings[0]['unit_ref'], mappings[1]['unit_ref'] = mappings[1]['unit_ref'], mappings[0]['unit_ref']
        elif fault['kind'] == 'selected_refs':
            value['selected_unit_refs'].reverse()
        elif fault['kind'] == 'unresolved_reason':
            mappings[1]['reason'] = 'A different missing-unit explanation.'
        else:
            raise ValueError('unknown synthetic regression')


def main():
    config = json.loads(Path(sys.argv[1]).read_text(encoding='utf-8'))
    spec = importlib.util.spec_from_file_location('native_replay_under_test', sys.argv[2])
    replay = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(replay)
    native_invoke = replay.invoke

    def invoke(cli, args, directory, payload=None):
        observation, stdout, stderr = native_invoke(cli, args, directory, payload)
        if args[0] == 'draft-video-coverage-input' and observation['exit_code'] == 0:
            source = json.loads(payload if payload is not None else (Path(directory) / args[1]).read_bytes())
            mappings = source['coverage']['mappings']
            if (mappings[0]['source_range'] == config['source_range']
                    and mappings[1]['status'] == config['second_status']
                    and not (config['fault'].get('preview_only') and '--emit-v1' in args)):
                document = json.loads(stdout)
                change(document, config['fault'])
                stdout = (json.dumps(document, separators=(',', ':'), sort_keys=True) + '\n').encode('utf-8')
                observation['stdout_base64'] = base64.b64encode(stdout).decode('ascii')
                observation['stdout_sha256'] = replay.sha(stdout)
        return observation, stdout, stderr

    replay.invoke = invoke
    sys.argv = sys.argv[2:]
    return replay.main()


if __name__ == '__main__':
    raise SystemExit(main())
