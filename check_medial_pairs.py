
consonants = ['b', 'd', 'g', 'j', 'v', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x', 'l', 'm', 'n', 'r']

# Forbidden follows for each consonant
# Based on !forbidden_chars in camxes.peg
forbidden = {
    'l': ['h', 'glide', 'l'],
    'm': ['h', 'glide', 'm', 'z'],
    'n': ['h', 'glide', 'n', 'tc', 'ts', 'dj', 'dz'], # 'affricate'
    'r': ['h', 'glide', 'r'],
    'b': ['h', 'glide', 'b', 'c', 'f', 'k', 'p', 's', 't', 'x'], # 'unvoiced'
    'd': ['h', 'glide', 'd', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'g': ['h', 'glide', 'g', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'v': ['h', 'glide', 'v', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'j': ['h', 'glide', 'j', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    'z': ['h', 'glide', 'z', 'j', 'c', 'f', 'k', 'p', 's', 't', 'x'],
    's': ['h', 'glide', 's', 'c', 'b', 'd', 'g', 'j', 'v', 'z'], # 'voiced'
    'c': ['h', 'glide', 'c', 's', 'x', 'b', 'd', 'g', 'j', 'v', 'z'],
    'x': ['h', 'glide', 'x', 'c', 'k', 'b', 'd', 'g', 'j', 'v', 'z'],
    'k': ['h', 'glide', 'k', 'x', 'b', 'd', 'g', 'j', 'v', 'z'],
    'f': ['h', 'glide', 'f', 'b', 'd', 'g', 'j', 'v', 'z'],
    'p': ['h', 'glide', 'p', 'b', 'd', 'g', 'j', 'v', 'z'],
    't': ['h', 'glide', 't', 'b', 'd', 'g', 'j', 'v', 'z']
}

def is_valid_pair(pair):
    c1, c2 = pair[0], pair[1]
    # Check if c2 is in forbidden[c1]
    # Note: 'h' and 'glide' are not in the consonant list, so we can ignore them.
    # What about 'affricate', 'unvoiced', 'voiced'?
    # We need to expand those.
    
    if c2 in forbidden[c1]: return False
    
    # Expand affricate, unvoiced, voiced in forbidden
    f_list = forbidden[c1]
    
    # Affricate
    if 'tc' in f_list and pair == 'tc': return False
    if 'ts' in f_list and pair == 'ts': return False
    if 'dj' in f_list and pair == 'dj': return False
    if 'dz' in f_list and pair == 'dz': return False
    
    # Unvoiced
    unvoiced = ['c', 'f', 'k', 'p', 's', 't', 'x']
    if 'unvoiced' in f_list and c2 in unvoiced: return False
    
    # Voiced
    voiced = ['b', 'd', 'g', 'j', 'v', 'z']
    if 'voiced' in f_list and c2 in voiced: return False
    
    return True

medial_pairs = []
for c1 in consonants:
    for c2 in consonants:
        pair = c1 + c2
        if is_valid_pair(pair):
            medial_pairs.append(pair)

print("Medial pairs:", ",".join(sorted(medial_pairs)))
