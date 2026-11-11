"""Report moderator-coded revision outcomes for the example collection (Python 3.11+)."""
import argparse
import csv
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
IDS = {"A", "B", "C"}
FIELDS = ("participant", "case", "language", "condition", "card", "received", "changed",
          "corrected", "collateral", "seconds", "finished")
RESULT = (*FIELDS[:5], "expected", "received", "unreturned", "received_unaffected",
          "missed", "unnecessary", "damaged", "exact", "seconds", "finished")


def ids(raw):
    parts = raw.split()
    if set(parts) - IDS or len(parts) != len(set(parts)):
        raise ValueError("use distinct space-separated draft IDs A, B, C or an empty cell")
    return set(parts)


def label(values):
    return " ".join(sorted(values))


def read_scores(path):
    with (HERE / "expected.csv").open(encoding="utf-8", newline="") as keyfile:
        expected = {(r["case"], r["card"]): ids(r["expected"])
                    for r in csv.DictReader(keyfile)}
    with path.open(encoding="utf-8-sig", newline="") as records:
        reader = csv.DictReader(records, strict=True)
        if tuple(reader.fieldnames or ()) != FIELDS:
            raise ValueError("use the header in revisions.csv")
        seen = set()
        for row in reader:
            try:
                if None in row or None in row.values():
                    raise ValueError("unexpected number of CSV fields")
                if not row["participant"].strip() or row["language"] not in {"en", "zh-TW"}:
                    raise ValueError("supply a participant code and language en or zh-TW")
                if row["condition"] not in {"notes", "mapped", "mapped-bounds"}:
                    raise ValueError("use condition notes, mapped or mapped-bounds")
                if row["finished"] not in {"yes", "no"} or int(row["seconds"]) < 0:
                    raise ValueError("supply nonnegative whole seconds and finished yes or no")
                key = (row["case"], row["card"])
                if key not in expected:
                    raise ValueError("unknown case or change card")
                session = tuple(row[k] for k in FIELDS[:5])
                if session in seen:
                    raise ValueError("duplicate participant/case/language/condition/card")
                seen.add(session)
                received = ids(row["received"])
                changed = ids(row["changed"])
                corrected = ids(row["corrected"])
                collateral = ids(row["collateral"])
                if row["finished"] == "yes" and received != IDS:
                    raise ValueError("a finished attempt must return all three drafts")
                if not changed <= received:
                    raise ValueError("changed drafts must have been received")
                if not corrected <= (changed & expected[key]) or not collateral <= changed:
                    raise ValueError("corrected must be an expected changed draft; collateral must be changed")
                missed = expected[key] - corrected
                extra = changed - expected[key]
                exact = row["finished"] == "yes" and not (missed or extra or collateral)
                yield (*session, label(expected[key]), label(received), label(IDS - received),
                       label(received - expected[key]), label(missed), label(extra), label(collateral),
                       "yes" if exact else "no", row["seconds"], row["finished"])
            except ValueError as exc:
                raise ValueError(f"line {reader.line_num}: {exc}") from exc


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("records", type=Path, help="completed private copy of revisions.csv")
    args = parser.parse_args()
    try:
        scores = list(read_scores(args.records))
    except (OSError, csv.Error, ValueError) as exc:
        parser.exit(2, f"{parser.prog}: {exc}\n")
    out = csv.writer(sys.stdout, lineterminator="\n")
    out.writerow(RESULT)
    out.writerows(scores)


if __name__ == "__main__":
    main()
