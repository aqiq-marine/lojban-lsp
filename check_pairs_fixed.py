
consonants = ['b', 'd', 'g', 'j', 'v', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x', 'l', 'm', 'n', 'r']

def is_liquid(c): return c in ['l', 'r']

def is_other(c, next_c):
    # other <- p / t !l / k / f / x / b / d !l / g / v / m / n !liquid
    # For a pair (C1, C2), next_c is C2, then a non-consonant.
    # When checking C2 as 'other', next_c is not-a-consonant.
    # So !l and !liquid are true.
    # BUT, if c1 is 'other', next_c is c2.
    
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
    # sibilant <- c / s !x / (j / z) !n !liquid
    if c == 'c': return True
    if c == 's': return next_c != 'x'
    if c in ['j', 'z']: return next_c != 'n' and not is_liquid(next_c)
    return False

def is_affricate(pair):
    return pair in ['tc', 'ts', 'dj', 'dz']

def is_initial(pair):
    c1, c2 = pair[0], pair[1]
    
    # 1. Affricate
    if is_affricate(pair): return True
    
    # 2. Sibilant? Other? Liquid?
    # Possible 2-consonant combinations:
    
    # S+O:
    # S(c1, c2) and O(c2, ' ')
    if is_sibilant(c1, c2) and is_other(c2, ' '): return True
    
    # S+L:
    # S(c1, c2) and L(c2)
    if is_sibilant(c1, c2) and is_liquid(c2): return True
    
    # O+L:
    # O(c1, c2) and L(c2)
    if is_other(c1, c2) and is_liquid(c2): return True
    
    return False

initial_pairs = []
for c1 in consonants:
    for c2 in consonants:
        pair = c1 + c2
        if is_initial(pair):
            initial_pairs.append(pair)

print("Initial pairs:", ",".join(sorted(initial_pairs)))
