"""Render the observed R1 statistics; no remote services or model calls."""
from pathlib import Path
import json
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
s = json.loads((ROOT / 'statistics_R1.json').read_text())
labels = ['Facts', 'Traceability', 'Uncertainty', 'Coherence', 'Clarity', 'Audience', 'Completion']
yy = np.arange(7)
matplotlib.rcParams['svg.hashsalt'] = 'storytelling-gemini-r1-20261113'
fig, (left, right) = plt.subplots(1, 2, figsize=(13, 5.7), gridspec_kw={'width_ratios': [1, 1.15]})
left.scatter([v['mean_X'] for v in s['dimensions']], yy - .08, label='X', color='#12687c', s=40)
left.scatter([v['mean_Y'] for v in s['dimensions']], yy + .08, label='Y', color='#c46a36', s=40)
left.set_yticks(yy, labels)
left.invert_yaxis()
left.set_xlim(1, 5.1)
left.set_xlabel('Mean rating (1-5)')
left.set_title('Observed X / Y scores | 40 paired cases')
left.legend(frameon=False)
means = np.array([v['mean_story_minus_control'] for v in s['dimensions']])
intervals = np.array([v['bootstrap_case_95_CI'] for v in s['dimensions']])
right.errorbar(means, yy, xerr=[means - intervals[:, 0], intervals[:, 1] - means], fmt='o', color='#5c4f8b', capsize=3)
right.axvline(0, color='#777', lw=1)
right.set_yticks(yy, labels)
right.invert_yaxis()
right.set_xlim(-.7, .4)
right.set_title('Declared Story - Control | labels unverified')
right.set_xlabel('Paired mean difference; marginal 95% case-bootstrap CI')
for ax in [left, right]:
    ax.grid(axis='x', alpha=.18)
    ax.spines[['top', 'right']].set_visible(False)
fig.suptitle('Gemini R1: exploratory results, one judge, X always first', fontsize=14)
fig.tight_layout()
(ROOT / 'assets').mkdir(exist_ok=True)
target = ROOT / 'assets/r1-comparison.svg'
fig.savefig(target, bbox_inches='tight', metadata={'Date': None})
target.write_text('\n'.join(line.rstrip() for line in target.read_text(encoding='utf-8').splitlines()) + '\n',encoding='utf-8',newline='\n')
plt.close(fig)
print('Wrote assets/r1-comparison.svg')
