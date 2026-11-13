"""Numeric transcription of the user's Gemini R1 records.

This is NOT a verbatim copy of the long JSON evidence/reasons. The original
chat submission remains authoritative for those strings. Rating order below:
factual_accuracy, source_traceability, uncertainty_boundary,
narrative_coherence, clarity, audience_fit, task_completion.
"""
DIMENSIONS = ['factual_accuracy','source_traceability','uncertainty_boundary',
              'narrative_coherence','clarity','audience_fit','task_completion']
TRANSCRIPTION = '''
01 5555555 4454444 X
02 5555555 5455554 X
03 3234543 4244443 Y
04 2334443 2354443 tie
05 5555555 5455554 X
06 2344543 4454444 Y
07 2334442 4455554 Y
08 5555555 5455554 X
09 4555554 2334442 X
10 5555555 5455554 X
11 2344443 3455554 Y
12 4455554 2254443 X
13 5455554 2234443 X
14 4454443 5555555 Y
15 5455554 5555555 Y
16 5445554 5355554 X
17 5555554 5554454 X
18 5545555 5455554 tie
19 5455555 5554455 X
20 4354443 5455554 Y
21 5455554 5555555 Y
22 5555555 5455554 X
23 5555555 4455454 X
24 5555554 3334443 X
25 5455554 4455554 X
26 5455554 5555555 Y
27 5555555 3444443 X
28 5555555 5555554 X
29 5555555 5555555 tie
30 5555555 4454454 X
31 5555555 5445554 X
32 4455554 3344443 X
33 5455555 3344443 X
34 5455555 4345454 X
35 5455554 4344443 X
36 5454454 4455554 X
37 4454454 5455555 Y
38 5455554 2344443 X
39 5445554 4445454 X
40 5555555 5455454 X
'''

CRITICAL_X = {3,4,6,7,11}
CRITICAL_Y = {3,4,9,11,12,13,24,27,33,38}

# C01-C10 assignment is available only in the compacted conversation summary.
# C11-C40 are the explicitly published keys in the prior batch replies.
# These are declared assignments, not verified generation-condition logs.
STORY_X = set(range(2,11,2)) | {11,14,16,18,20} | {21,23,26,27,29} | {31,33,36,38,40}

TOPICS = [
 '地方選舉與深偽宣導','10月進出口','鴻海財報與供應壓力','AI相關資安事件',
 '流感通報','登革熱病例','台美研發平台','台灣與巴紐代表處','布魯克林倒塌','COP31與溫度',
 '基底核細胞圖譜','PFAS與乳癌','達卡青年奧運跆拳道','COPD風險','APEC代表',
 '挪威運動員入境政策','AI素養競賽','臺灣科學節','臺北世界郵展','烏克蘭會談',
 'Quantum Taiwan','10月稅收','剛果埃博拉','原油市場','G7聲明','核心關鍵技術草案',
 '納瑟醫院空襲','美國期中選舉','台海軍事通報','社造盛市','印尼野火歸因',
 '哥委綁架案合作','Asturias骨灰返國','航空調查公開率','美墨關稅威脅',
 '無名戰士墓哨兵勳章','WHO結核病','HPTN094行動醫療','Ofcom與Meta','新北耶誕城']

def cases():
    out=[]
    for line in TRANSCRIPTION.strip().splitlines():
        num,x,y,pref=line.split(); n=int(num)
        out.append(dict(case_id=f'C{n:02d}', original_case_id=f'C{n:02d}' if n<=10 else str(n),
                        case_number=n, batch=(n-1)//10+1, topic=TOPICS[n-1],
                        run_id=f'R1_batch{(n-1)//10+1}_{((n-1)//10+1)*10}of40',
                        presentation_order='X_then_Y',
                        ratings_X=list(map(int,x)),ratings_Y=list(map(int,y)),
                        critical_X=n in CRITICAL_X,critical_Y=n in CRITICAL_Y,
                        preference=pref,declared_story_variant='X' if n in STORY_X else 'Y',
                        assignment_provenance='conversation_summary_only' if n<=10 else 'published_batch_answer_key',
                        assignment_verification='no_generation_manifest_available'))
    return out
