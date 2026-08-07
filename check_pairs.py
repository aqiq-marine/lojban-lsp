
consonants = ['b', 'd', 'g', 'j', 'v', 'z', 'c', 'f', 'k', 'p', 's', 't', 'x', 'l', 'm', 'n', 'r']

def is_liquid(c):
    return c in ['l', 'r']

# other <- p / t !l / k / f / x / b / d !l / g / v / m / n !liquid
# Since the pair is followed by !consonant, the next char is never l, r, etc.
# So "other" is just:
def is_other(c):
    return c in ['p', 't', 'k', 'f', 'x', 'b', 'd', 'g', 'v', 'm', 'n']

# sibilant <- c / s !x / (j / z) !n !liquid
# Same logic.
def is_sibilant(c):
    return c in ['c', 's', 'j', 'z']

def is_affricate(pair):
    return pair in ['tc', 'ts', 'dj', 'dz']

def is_initial(pair):
    c1, c2 = pair[0], pair[1]
    
    # 1. Affricate
    if is_affricate(pair): return True
    
    # 2. Sibilant? Other? Liquid?
    # Possible 2-consonant combinations:
    
    # S+O:
    if is_sibilant(c1) and is_other(c2): return True
    
    # S+L:
    if is_sibilant(c1) and is_liquid(c2): return True
    
    # O+L:
    if is_other(c1) and is_liquid(c2): return True
    
    # Any others?
    # What about just S+something else?
    # Or just O+something?
    # Or just L? (Not 2 consonants)
    
    return False

initial_pairs = []
for c1 in consonants:
    for c2 in consonants:
        pair = c1 + c2
        if is_initial(pair):
            initial_pairs.append(pair)

print("Initial pairs:", ",".join(sorted(initial_pairs)))
