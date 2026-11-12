# SPDX-License-Identifier: MIT
"""Private, single-facilitator N-07 recorder. No editorial scoring or approval."""
import argparse
import hashlib
import json
import math
import os
import queue
import random
import re
import secrets
import sqlite3
import subprocess
import sys
import threading
import time
import uuid
from contextlib import contextmanager
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PROTOCOL = 'n07-native-notes-v3-proposed'
PROTOCOL_SHA = 'a9b3038272107ee6986c22879c354e73cbbd55d40d498ad3d3c12aef87d0bb1d'
SOURCE = 'ce73836caa86502933f538821fdbf5dac92c1224'
MATERIALS = '8e2c08094f6bce70a7507e11ed84265bd98e3820'
MAX_INPUT = 16 * 1024 * 1024
MAX_OUTPUT = 16 * 1024 * 1024  # per stream; overflow is explicitly partial
LIMIT = 480
SAFE = re.compile(r'[A-Za-z0-9][A-Za-z0-9_-]{0,63}\Z')
SHA = re.compile(r'[a-f0-9]{64}\Z')


def utc():
    return datetime.now(timezone.utc).isoformat()


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def file_hash(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for chunk in iter(lambda: f.read(65536), b''):
            h.update(chunk)
    return h.hexdigest()


def read_bytes(path, limit=MAX_INPUT):
    path = Path(path)
    if not path.is_file():
        raise ValueError('input must be a regular file')
    with path.open('rb') as f:
        raw = f.read(limit + 1)
    if len(raw) > limit:
        raise ValueError('input exceeds byte limit')
    return raw


def unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('duplicate JSON field')
        result[key] = value
    return result


def invalid_constant(value):
    raise ValueError('non-finite JSON number')


def finite_float(value):
    number = float(value)
    if not math.isfinite(number):
        raise ValueError('non-finite JSON number')
    return number


def parse(raw):
    try:
        if isinstance(raw, bytes):
            raw = raw.decode('utf-8')
        return json.loads(raw, object_pairs_hook=unique, parse_constant=invalid_constant,
                          parse_float=finite_float)
    except (UnicodeError, RecursionError, json.JSONDecodeError) as exc:
        raise ValueError('invalid bounded UTF-8 JSON') from exc


def read_json(path):
    return parse(read_bytes(path, 1024 * 1024))


def fields(value, expected):
    if not isinstance(value, dict) or set(value) != set(expected.split()):
        raise ValueError('unexpected or missing JSON fields')


def timestamp(value):
    if not isinstance(value, str):
        raise ValueError('timestamp must be a string with timezone')
    try:
        date = datetime.fromisoformat(value.replace('Z', '+00:00'))
    except ValueError as exc:
        raise ValueError('invalid timestamp') from exc
    if date.tzinfo is None:
        raise ValueError('timestamp needs timezone')


def code(value):
    if not isinstance(value, str) or not SAFE.fullmatch(value):
        raise ValueError('use a bounded anonymous code')
    return value


def text(value):
    if not isinstance(value, str) or not value.strip() or len(value) > 4096:
        raise ValueError('nonempty text of at most 4096 characters required')
    return value


@contextmanager
def transaction(folder):
    db = sqlite3.connect((Path(folder).resolve() / 'records.sqlite3').as_uri() + '?mode=rw',
                         uri=True, timeout=5)
    try:
        db.execute('PRAGMA synchronous=FULL')
        db.execute('BEGIN IMMEDIATE')
        yield db
        db.commit()
    except BaseException:
        db.rollback()
        raise
    finally:
        db.close()


def append(db, kind, payload):
    db.execute('INSERT INTO events(kind,payload) VALUES (?,?)',
               (kind, json.dumps({'recorded_utc': utc(), **payload}, ensure_ascii=False,
                                 allow_nan=False)))


def blob(db, raw):
    sha = digest(raw)
    db.execute('INSERT OR IGNORE INTO blobs(sha,bytes) VALUES (?,?)', (sha, raw))
    if db.execute('SELECT bytes FROM blobs WHERE sha=?', (sha,)).fetchone()[0] != raw:
        raise ValueError('stored artifact hash mismatch')
    return sha


def state(db):
    config = parse(db.execute('SELECT value FROM config').fetchone()[0])
    result = {'config': config, 'participants': {}, 'sessions': {}, 'calls': {},
              'events': [], 'comparable_pairs': None, 'recommendation': '待真人證據',
              'maintainer_decision': None, 'human_review_required': True}
    for seq, kind, payload in db.execute('SELECT seq,kind,payload FROM events ORDER BY seq'):
        p = parse(payload)
        result['events'].append({'seq': seq, 'kind': kind, **p})
        if kind == 'enroll':
            result['participants'][p['participant']] = p
        elif kind == 'session_start':
            result['sessions'][p['session']] = {**p, 'status': 'active',
                'analysis_seconds': None, 'elapsed_seconds': None, 'completed': None}
        elif kind == 'session_end':
            result['sessions'][p['session']].update(p)
        elif kind == 'call_start':
            result['calls'][p['call']] = {**p, 'status': 'unknown'}
        elif kind == 'call_result':
            result['calls'][p['call']].update(p)
    return result


def inspect(folder):
    with transaction(folder) as db:
        return state(db)


def active(s, session):
    if session not in s['sessions'] or s['sessions'][session]['status'] != 'active':
        raise ValueError('session is absent or already ended; no late state overwrite')
    return s['sessions'][session]


def initialize(folder, cli, receipt_path, mode, agreement_path):
    folder, cli = Path(folder).resolve(), Path(cli).resolve()
    if folder.is_relative_to(ROOT):
        raise ValueError('keep participant records outside this repository')
    if mode not in {'rehearsal', 'trial'}:
        raise ValueError('choose rehearsal or trial')
    receipt_raw = read_bytes(receipt_path, 1024 * 1024)
    r = parse(receipt_raw)
    fields(r, 'protocol human_evidence source_commit materials_commit binary_sha256 lock_sha256 '
              'raw_report_sha256 engineering_results_sha256 rustc cargo')
    if (r['protocol'] != PROTOCOL or r['human_evidence'] is not False
            or r['source_commit'] != SOURCE or r['materials_commit'] != MATERIALS):
        raise ValueError('receipt does not declare the frozen v3 build')
    for field in ('binary_sha256', 'lock_sha256', 'raw_report_sha256', 'engineering_results_sha256'):
        if not isinstance(r[field], str) or not SHA.fullmatch(r[field]):
            raise ValueError('invalid receipt digest')
    text(r['rustc']); text(r['cargo'])
    if file_hash(cli) != r['binary_sha256']:
        raise ValueError('executable differs from build receipt')
    protocol = read_bytes(ROOT / 'docs/validation/n07-native-v3.md')
    if digest(protocol) != PROTOCOL_SHA:
        raise ValueError('protocol changed; review and version recorder before recruitment')
    agreement_raw = None
    if mode == 'trial':
        if agreement_path is None:
            raise ValueError('trial requires the actual prospective agreement record')
        agreement_raw = read_bytes(agreement_path, 1024 * 1024)
        a = parse(agreement_raw)
        fields(a, 'schema_version protocol_sha256 protocol_commit recorder_sha256 adopted recorded_at maintainer_label round_id')
        if (type(a['schema_version']) is not int or a['schema_version'] != 1
                or a['protocol_sha256'] != PROTOCOL_SHA or a['adopted'] is not True
                or a['recorder_sha256'] != file_hash(__file__)
                or a['protocol_commit'] != 'fe2b145250b42564bf5b4d781ced5a95e8a8f9bf'):
            raise ValueError('agreement must identify and adopt the frozen prospective protocol')
        timestamp(a['recorded_at']); text(a['maintainer_label']); code(a['round_id'])
        if datetime.fromisoformat(a['recorded_at'].replace('Z', '+00:00')) > datetime.now(timezone.utc):
            raise ValueError('agreement cannot be in the future')
    elif agreement_path is not None:
        raise ValueError('rehearsal cannot import an agreement or become a trial')
    seed = secrets.token_hex(16)
    allocation = ['AB', 'AB', 'AB', 'BA', 'BA']
    random.Random(seed).shuffle(allocation)
    config = {'schema_version': 1, 'mode': mode, 'created_utc': utc(),
              'protocol': PROTOCOL, 'protocol_sha256': PROTOCOL_SHA,
              'source_commit_declared': SOURCE, 'materials_commit_declared': MATERIALS,
              'source_binding_verified': False, 'agreement_authenticated': False,
              'cli': str(cli), 'binary_sha256': r['binary_sha256'],
              'seed_string': seed, 'allocation': allocation,
              'random_method': 'random.Random(seed_string).shuffle', 'python': sys.version,
              'receipt_sha256': digest(receipt_raw), 'recorder_sha256': file_hash(__file__),
              'agreement_sha256': digest(agreement_raw) if agreement_raw is not None else None}
    folder.mkdir(mode=0o700, parents=False, exist_ok=False)
    db = sqlite3.connect(folder / 'records.sqlite3')
    try:
        db.execute('PRAGMA synchronous=FULL')
        db.executescript('CREATE TABLE config(value TEXT NOT NULL);'
                        'CREATE TABLE events(seq INTEGER PRIMARY KEY,kind TEXT NOT NULL,payload TEXT NOT NULL);'
                        'CREATE TABLE blobs(sha TEXT PRIMARY KEY,bytes BLOB NOT NULL);')
        db.execute('INSERT INTO config VALUES (?)', (json.dumps(config),))
        blob(db, receipt_raw); blob(db, protocol)
        if agreement_raw is not None:
            blob(db, agreement_raw)
        db.commit()
    finally:
        db.close()


def enroll(folder, participant, slot, screening_path, replacement_for=None):
    code(participant)
    if type(slot) is not int or not 1 <= slot <= 5:
        raise ValueError('slot must be 1 through 5')
    raw = read_bytes(screening_path, 1024 * 1024)
    p = parse(raw)
    fields(p, 'schema_version screened_at prior_exposure cli_json_experience facilitator_code')
    if type(p['schema_version']) is not int or p['schema_version'] != 1 or type(p['prior_exposure']) is not bool:
        raise ValueError('screening version and exposure must be explicit')
    timestamp(p['screened_at']); text(p['cli_json_experience']); code(p['facilitator_code'])
    with transaction(folder) as db:
        s = state(db)
        if participant in s['participants']:
            raise ValueError('anonymous code already used')
        occupants = [v for v in s['participants'].values() if v['slot'] == slot]
        if occupants:
            if replacement_for != occupants[-1]['participant']:
                raise ValueError('occupied slot requires its latest withdrawn/interrupted participant')
            old = [v for v in s['sessions'].values() if v['participant'] == replacement_for]
            if not old or old[-1]['status'] not in {'withdrawn', 'interrupted', 'clock_uncertain'}:
                raise ValueError('do not replace failed or timeout pairs to improve outcomes')
            if sum(v['replacement_for'] is not None for v in s['participants'].values()) >= 5:
                raise ValueError('five-replacement cap reached; new prospective decision needed')
        elif replacement_for is not None:
            raise ValueError('replacement target absent')
        append(db, 'enroll', {**p, 'participant': participant, 'slot': slot,
                             'assigned_order': s['config']['allocation'][slot - 1],
                             'replacement_for': replacement_for, 'screening_sha256': blob(db, raw)})


def start(folder, participant, condition):
    with transaction(folder) as db:
        s = state(db)
        if any(v['status'] == 'active' for v in s['sessions'].values()):
            raise ValueError('another session is active; never auto-resume or auto-retry it')
        if participant not in s['participants'] or s['participants'][participant]['prior_exposure']:
            raise ValueError('fresh enrolled author required')
        p = s['participants'][participant]
        if any(v['replacement_for'] == participant for v in s['participants'].values()):
            raise ValueError('participant has been replaced')
        previous = [v for v in s['sessions'].values() if v['participant'] == participant]
        if (len(previous) >= 2 or (previous and previous[-1]['status'] not in {'submitted', 'timeout'})
                or condition != p['assigned_order'][len(previous)]):
            raise ValueError('condition differs from allocation or participant cannot continue')
        sid = uuid.uuid4().hex
        append(db, 'session_start', {'session': sid, 'participant': participant, 'condition': condition,
                                     'period': len(previous) + 1, 'start_utc': utc(), 'limit_seconds': LIMIT})
        return sid


def workspace(folder, participant, condition, destination):
    """Copy only hash-pinned participant files, never the facilitator key."""
    destination = Path(destination).resolve()
    if destination.is_relative_to(ROOT) or condition not in {'A', 'B'}:
        raise ValueError('use A/B and a private workspace outside the repository')
    manifest_raw = read_bytes(ROOT / 'docs/validation/n07-pilot/material-sha256.json')
    if digest(manifest_raw) != 'e7c4f610811eb90ec8a46597bfdc8093ad81ae2bc21c7ea0ababbe82a375c42a':
        raise ValueError('historical material receipt changed')
    manifest = parse(manifest_raw)
    prefix = 'examples/evaluation/n07-pilot/participant-kit/'
    files = {}
    for path, expected in manifest.items():
        if path.startswith(prefix + 'shared/') or (condition == 'B' and path.startswith(prefix + 'cli/')):
            raw = read_bytes(ROOT / path)
            if digest(raw) != expected:
                raise ValueError('participant material differs from frozen receipt')
            files[path[len(prefix):]] = raw
    if len(files) != (5 if condition == 'A' else 10):
        raise ValueError('participant packet incomplete')
    with transaction(folder) as db:
        s = state(db)
        if participant not in s['participants']:
            raise ValueError('enroll the anonymous author first')
        if any(v['participant'] == participant and v['condition'] == condition for v in s['sessions'].values()):
            raise ValueError('prepare matched starting work before that arm starts')
        destination.mkdir(mode=0o700, exist_ok=False)
        identities = {}
        for path, raw in files.items():
            target = destination / path
            target.parent.mkdir(exist_ok=True)
            write_new(target, raw)
            identities[path] = blob(db, raw)
        append(db, 'workspace', {'participant': participant, 'condition': condition,
                                'destination': str(destination), 'input_sha256': identities,
                                'materials_commit': MATERIALS})


def end(folder, session, status, elapsed, artifacts, reason):
    if status not in {'submitted', 'timeout', 'withdrawn', 'interrupted', 'clock_uncertain'}:
        raise ValueError('unsupported end status')
    if elapsed is not None and (isinstance(elapsed, bool) or not math.isfinite(elapsed) or elapsed < 0):
        raise ValueError('invalid measured time')
    if status not in {'interrupted', 'clock_uncertain'} and elapsed is None:
        raise ValueError('measured elapsed time required')
    if status == 'submitted' and elapsed >= LIMIT:
        status = 'timeout'
    with transaction(folder) as db:
        active(state(db), session)
        append(db, 'session_end', {'session': session, 'status': status, 'stop_utc': utc(),
            'elapsed_seconds': elapsed, 'analysis_seconds': LIMIT if status == 'timeout' else
            elapsed if status == 'submitted' else None,
            'completed': False if status == 'timeout' else None, 'artifacts': artifacts, 'reason': reason})
    return status


def recover(folder, session, reason):
    text(reason)
    end(folder, session, 'interrupted', None, [], reason)


def attach(folder, session, kind, path):
    if kind not in {'submission', 'feedback', 'scoring', 'deviation', 'decision'}:
        raise ValueError('unsupported record kind')
    raw = read_bytes(path)
    with transaction(folder) as db:
        if session not in state(db)['sessions']:
            raise ValueError('session absent')
        sha = blob(db, raw)
        append(db, 'attachment', {'session': session, 'attachment_kind': kind,
                                  'original_name': Path(path).name, 'sha256': sha,
                                  'size': len(raw), 'content_is_unassessed': True})
    return sha


def write_new(path, raw):
    with Path(path).open('xb') as f:
        f.write(raw); f.flush(); os.fsync(f.fileno())


def capture(argv, stdout_path, stderr_path, timeout):
    """Bound each raw stream; partial files survive recorder termination."""
    started = time.perf_counter()
    overflow = threading.Event()
    io_errors = []
    # Create files before launch, so even a failed spawn has raw empty streams.
    out, err = Path(stdout_path).open('xb'), Path(stderr_path).open('xb')
    child = None
    try:
        try:
            child = subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                     stdin=subprocess.DEVNULL, shell=False)
        except OSError as exc:
            return {'exit_code': None, 'timed_out': False, 'launch_error': type(exc).__name__,
                    'output_limited': False, 'capture_error': None,
                    'process_ms': (time.perf_counter() - started) * 1000}

        def pump(source, target):
            count = 0
            try:
                while True:
                    chunk = source.read(65536)
                    if not chunk:
                        break
                    remaining = MAX_OUTPUT - count
                    target.write(chunk[:remaining]); target.flush(); os.fsync(target.fileno())
                    count += min(len(chunk), remaining)
                    if len(chunk) > remaining:
                        overflow.set()
                        child.kill()
                        break
            except OSError as exc:
                io_errors.append(type(exc).__name__)
                child.kill()
            finally:
                source.close()

        threads = [threading.Thread(target=pump, args=pair, daemon=True)
                   for pair in [(child.stdout, out), (child.stderr, err)]]
        for t in threads:
            t.start()
        timed_out = False
        try:
            child.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            child.kill(); child.wait()
        for t in threads:
            t.join(timeout=2)
        if any(t.is_alive() for t in threads):
            # The pinned native CLI has no descendants; inherited pipes violate this boundary.
            raise ValueError('capture did not end; preserve unknown call and recover session')
        return {'exit_code': child.returncode, 'timed_out': timed_out,
                'launch_error': None, 'output_limited': overflow.is_set(),
                'capture_error': io_errors or None,
                'process_ms': (time.perf_counter() - started) * 1000}
    finally:
        if child is not None and child.poll() is None:
            child.kill(); child.wait()
        out.close(); err.close()


def invoke(folder, session, args, timeout):
    if (not args or args[0] not in {'render-narrative', 'audit-copy-claims'}
            or len(args) != (2 if args[0] == 'render-narrative' else 3)):
        raise ValueError('only render-narrative INPUT and audit-copy-claims COPY LEDGER allowed')
    if isinstance(timeout, bool) or not math.isfinite(timeout) or not 0 < timeout <= 60:
        raise ValueError('process timeout must be within remaining author time, at most 60 seconds')
    folder = Path(folder).resolve()
    originals = [Path(p).resolve() for p in args[1:]]
    raw = [read_bytes(p) for p in originals]
    call = uuid.uuid4().hex
    directory = folder / 'calls' / call
    with transaction(folder) as db:
        s = state(db)
        if active(s, session)['condition'] != 'B':
            raise ValueError('native commands are unavailable in notes condition A')
        cli = s['config']['cli']
        before = file_hash(cli)
        if before != s['config']['binary_sha256']:
            raise ValueError('executable changed; do not launch')
        directory.mkdir(parents=True, exist_ok=False)
        snapshots = []
        for i, content in enumerate(raw):
            path = directory / ('input-' + str(i) + '.json')
            write_new(path, content)
            snapshots.append(str(path))
        argv = [cli, args[0], *snapshots]
        payload = {'call': call, 'session': session, 'argv': argv,
                   'original_inputs': [str(p) for p in originals],
                   'input_sha256': [blob(db, b) for b in raw], 'binary_sha256_before': before,
                   'stdout_file': str(Path('calls') / call / 'stdout'),
                   'stderr_file': str(Path('calls') / call / 'stderr'), 'start_utc': utc()}
        append(db, 'call_start', payload)
    # The call-start transaction is committed before the native process can run.
    measured = capture(argv, directory / 'stdout', directory / 'stderr', timeout)
    try:
        after = file_hash(cli)
    except OSError:
        after = None
    unchanged = []
    for p, content in zip(originals, raw):
        try:
            unchanged.append(file_hash(p) == digest(content))
        except OSError:
            unchanged.append(False)
    snapshots_unchanged = [file_hash(p) == digest(b) for p, b in zip(snapshots, raw)]
    with transaction(folder) as db:
        s = state(db)
        late = s['sessions'][session]['status'] != 'active'
        result = {**measured, 'call': call, 'session': session, 'stop_utc': utc(),
                  'status': 'captured', 'late_after_session_end': late,
                  'stdout_path': str(directory / 'stdout'), 'stderr_path': str(directory / 'stderr'),
                  'binary_sha256_after': after, 'binary_unchanged': before == after,
                  'original_inputs_unchanged': unchanged, 'snapshots_unchanged': snapshots_unchanged,
                  'stdout_sha256': blob(db, read_bytes(directory / 'stdout', MAX_OUTPUT)),
                  'stderr_sha256': blob(db, read_bytes(directory / 'stderr', MAX_OUTPUT))}
        append(db, 'call_result', result)
    return result


def export(folder, destination):
    destination = Path(destination).resolve()
    if destination.is_relative_to(ROOT) or destination.is_relative_to(Path(folder).resolve()):
        raise ValueError('export to a new private directory outside source and round')
    with transaction(folder) as db:
        s = state(db)
        artifacts = db.execute('SELECT sha,bytes FROM blobs').fetchall()
        for sha, raw in artifacts:
            if digest(raw) != sha:
                raise ValueError('stored artifact hash mismatch')
        s['exported_utc'] = utc()
        s['raw_partial_streams'] = []
        for call, value in s['calls'].items():
            if value['status'] == 'unknown':
                for stream in ('stdout', 'stderr'):
                    path = Path(folder) / 'calls' / call / stream
                    if path.is_file():
                        raw = read_bytes(path, MAX_OUTPUT)
                        sha = digest(raw)
                        artifacts.append((sha, raw))
                        s['raw_partial_streams'].append({'call': call, 'stream': stream,
                                                        'sha256': sha, 'complete': False})
        destination.mkdir(mode=0o700, exist_ok=False)
        (destination / 'blobs').mkdir()
        for sha, raw in dict(artifacts).items():
            write_new(destination / 'blobs' / sha, raw)
        write_new(destination / 'report.json', (json.dumps(s, ensure_ascii=False, indent=2,
                                                          allow_nan=False) + '\n').encode())


def session_loop(folder, participant, condition):
    monotonic_start, wall_start = time.perf_counter(), time.time()
    sid = start(folder, participant, condition)
    incoming = queue.Queue(maxsize=16)

    def reader():
        while True:
            line = sys.stdin.readline(16385)
            incoming.put(line)
            if not line:
                break

    threading.Thread(target=reader, daemon=True).start()
    print(json.dumps({'session': sid, 'limit_seconds': LIMIT}), flush=True)
    print('Timer running. Commands: render ["input"] | audit ["copy","ledger"] | '
          'submit ["file",...] | withdraw reason', file=sys.stderr, flush=True)
    try:
        while True:
            elapsed = time.perf_counter() - monotonic_start
            if abs((time.time() - wall_start) - elapsed) > 2:
                end(folder, sid, 'clock_uncertain', None, [], 'wall/monotonic clock divergence')
                return 1
            if elapsed >= LIMIT:
                end(folder, sid, 'timeout', elapsed, [], None)
                return 1
            active(inspect(folder), sid)
            try:
                line = incoming.get(timeout=min(0.25, LIMIT - elapsed))
            except queue.Empty:
                continue
            if not line:
                end(folder, sid, 'interrupted', None, [], 'input closed; elapsed not accepted')
                return 1
            try:
                if len(line) > 16384:
                    raise ValueError('command line too long')
                action, _, argument = line.strip().partition(' ')
                if action == 'withdraw':
                    end(folder, sid, 'withdrawn', time.perf_counter() - monotonic_start, [], text(argument))
                    return 0
                paths = parse(argument)
                if (not isinstance(paths, list) or not 1 <= len(paths) <= 16
                        or not all(isinstance(p, str) and 0 < len(p) <= 4096 for p in paths)):
                    raise ValueError('use a JSON array of 1–16 file paths')
                if action in {'render', 'audit'}:
                    command = 'render-narrative' if action == 'render' else 'audit-copy-claims'
                    remaining = LIMIT - (time.perf_counter() - monotonic_start)
                    if remaining <= 0:
                        continue
                    result = invoke(folder, sid, [command, *paths], min(60, remaining))
                    print(json.dumps(result), flush=True)
                elif action == 'submit':
                    hashes = [attach(folder, sid, 'submission', p) for p in paths]
                    elapsed = time.perf_counter() - monotonic_start
                    if abs((time.time() - wall_start) - elapsed) > 2:
                        end(folder, sid, 'clock_uncertain', None, hashes, 'clock divergence while saving')
                        return 1
                    status = end(folder, sid, 'submitted', elapsed, hashes, None)
                    return 0 if status == 'submitted' else 1
                else:
                    raise ValueError('unknown session command')
            except (ValueError, OSError) as exc:
                print(str(exc), file=sys.stderr, flush=True)
                with transaction(folder) as db:
                    append(db, 'command_error', {'session': sid, 'error': str(exc)})
    except KeyboardInterrupt:
        end(folder, sid, 'interrupted', None, [], 'keyboard interruption')
        return 1


def main():
    p = argparse.ArgumentParser(description=__doc__)
    commands = p.add_subparsers(dest='action', required=True)
    for name in ('init', 'enroll', 'workspace', 'session', 'recover', 'attach', 'status', 'export'):
        q = commands.add_parser(name)
        q.add_argument('--round', type=Path, required=True)
        if name == 'init':
            q.add_argument('--cli', type=Path, required=True)
            q.add_argument('--receipt', type=Path, required=True)
            q.add_argument('--mode', choices=['rehearsal', 'trial'], required=True)
            q.add_argument('--agreement', type=Path)
        elif name == 'enroll':
            q.add_argument('--participant', required=True)
            q.add_argument('--slot', type=int, required=True)
            q.add_argument('--screening', type=Path, required=True)
            q.add_argument('--replacement-for')
        elif name in {'session', 'workspace'}:
            q.add_argument('--participant', required=True)
            q.add_argument('--condition', choices=['A', 'B'], required=True)
            if name == 'workspace':
                q.add_argument('--out', type=Path, required=True)
        elif name in {'attach', 'recover'}:
            q.add_argument('--session', required=True)
            if name == 'recover':
                q.add_argument('--reason', required=True)
            else:
                q.add_argument('--kind', required=True)
                q.add_argument('--file', type=Path, required=True)
        elif name == 'export':
            q.add_argument('--out', type=Path, required=True)
    a = p.parse_args()
    try:
        if a.action == 'init':
            initialize(a.round, a.cli, a.receipt, a.mode, a.agreement)
        elif a.action == 'enroll':
            enroll(a.round, a.participant, a.slot, a.screening, a.replacement_for)
        elif a.action == 'session':
            return session_loop(a.round, a.participant, a.condition)
        elif a.action == 'workspace':
            workspace(a.round, a.participant, a.condition, a.out)
        elif a.action == 'recover':
            recover(a.round, a.session, a.reason)
        elif a.action == 'attach':
            attach(a.round, a.session, a.kind, a.file)
        elif a.action == 'export':
            export(a.round, a.out)
        # ASCII JSON escapes preserve Unicode on redirected Windows terminals too.
        print(json.dumps(inspect(a.round)))
        return 0
    except (ValueError, OSError, sqlite3.Error) as exc:
        print(json.dumps({'ok': False, 'error': str(exc)}), file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.stdin.reconfigure(encoding='utf-8')
    sys.exit(main())
