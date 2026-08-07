
consonants = ['b', 'd', 'g', 'j', 'v', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x', 'l', 'm', 'n', 'r']

# (Paste the logic from check_pairs_fixed.py to get initial_pairs)
# (Paste the logic from check_medial_pairs.py to get all_valid_pairs)

def is_liquid(c): return c in ['l', 'r']
def is_other(c, next_c):
    if c == 'p': return True
    if c == 't': return next_c != 'l'
    if c == 'k': return True
    if c == 'f': return True
    if c == 'x': return True
    if c == 'b': return True
    if c == 'd': return next_c != 'l'
    if c == 'g': return True
    if c == 'v': return True
    if c == 'm': return True
    if c == 'n': return not is_liquid(next_c)
    return False
def is_sibilant(c, next_c):
    if c == 'c': return True
    if c == 's': return next_c != 'x'
    if c in ['j', 'z']: return next_c != 'n' and not is_liquid(next_c)
    return False
def is_affricate(pair): return pair in ['tc', 'ts', 'dj', 'dz']

def is_initial(pair):
    c1, c2 = pair[0], pair[1]
    if is_affricate(pair): return True
    if is_sibilant(c1, c2) and is_other(c2, ' '): return True
    if is_sibilant(c1, c2) and is_liquid(c2): return True
    if is_other(c1, c2) and is_liquid(c2): return True
    return False

# Medial Logic
forbidden = {
    'l': ['h', 'glide', 'l'],
    'm': ['h', 'glide', 'm', 'z'],
    'n': ['h', 'glide', 'n', 'tc', 'ts', 'dj', 'dz'],
    'r': ['h', 'glide', 'r'],
    'b': ['h', 'glide', 'b', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'd': ['h', 'glide', 'd', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'g': ['h', 'glide', 'g', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'v': ['h', 'glide', 'v', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'j': ['h', 'glide', 'j', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'z': ['h', 'glide', 'z', 'j', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    's': ['h', 'glide', 's', 'c', 'b', 'd', 'g', 'j', 'v', 'z'],
    'c': ['h', 'glide', 'c', 's', 'x', 'b', 'd', 'g', 'j', 'v', 'z'],
    'x': ['h', 'glide', 'x', 'c', 'k', 'b', 'd', 'g', 'j', 'v', 'z'],
    'k': ['h', 'glide', 'k', 'x', 'b', 'd', 'g', 'j', 'v', 'z'],
    'f': ['h', 'glide', 'f', 'b', 'd', 'g', 'j', 'v', 'z'],
    'p': ['h', 'glide', 'p', 'b', 'd', 'g', 'j', 'v', 'z'],
    't': ['h', 'glide', 't', 'b', 'd', 'g', 'j', 'v', 'z']
}
def is_valid_pair(pair):
    c1, c2 = pair[0], pair[1]
    if c2 in forbidden[c1]: return False
    f_list = forbidden[c1]
    if 'tc' in f_list and pair == 'tc': return False
    if 'ts' in f_list and pair == 'ts': return False
    if 'dj' in f_list and pair == 'dj': return False
    if 'dz' in f_list and pair == 'dz': return False
    unvoiced = ['c', 'f', 'k', 'p', 's', 't', 'x']
    if 'unvoiced' in f_list and c2 in unvoiced: return False
    voiced = ['b', 'd', 'g', 'j', 'v', 'z']
    if 'voiced' in f_list and c2 in voiced: return False
    return True

initial_pairs = []
medial_pairs = []
for c1 in consonants:
    for c2 in consonants:
        pair = c1 + c2
        if is_initial(pair):
            initial_pairs.append(pair)
        if is_valid_pair(pair):
            # If it's valid, is it ALSO initial?
            # The prompt asks for initial AND medial.
            # Usually they are distinct sets in linguistic analysis.
            # Let me just output both sets.
            medial_pairs.append(pair)

print("Initial pairs:", ",".join(sorted(initial_pairs)))
print("Medial pairs:", ",".join(sorted(medial_pairs)))
