import json
import random
from pathlib import Path
from string import Template

RANDOM_SEED = 23571637
MAX_INT = 1 << 30
N_SEED = 8


def generate_hash_seed() -> list[int]:
    """Generates hash for hash function."""
    random.seed(RANDOM_SEED)
    seeds = [ random.randint(i, MAX_INT) for i in range(N_SEED) ]
    return seeds


def hash_function(s: str, a: list[int]) -> int:
    """Generate hash value."""
    hash_value = RANDOM_SEED;
    for i, c in enumerate(s):
        hash_value ^= ord(c) * a[i%N_SEED]
    return hash_value


def rust_hash_function() -> str:
    content = ""
    with open(Path('scripts', 'treesitter_nodes_src.rs'), 'r') as f:
        content = f.read()

    template = Template(content)
    rust_function = template.substitute(random_seed=RANDOM_SEED, n_seed=N_SEED)
    return rust_function


def generate_nodes(node_types: dict):
    seeds = generate_hash_seed()
    named_nodes = [node['type'] for node in node_types if node['named']]
    named_nodes.sort()

    init_seed_line = "static INIT_SEED: u32 = {};".format(RANDOM_SEED)
    seed_line = "const SEEDS: &'static [u32] = &[{}];".format(', '.join(str(s) for s in seeds))

    lines = []
    for node in named_nodes:
        const_name = node.upper()
        if const_name[0] == '_':
            const_name = const_name[1:]

        node_hash = hash_function(node, seeds)
        
        line = 'pub const {}: u32 = {}; // "{}"'.format(const_name, node_hash, node)
        lines.append(line)

    with open(Path('src', 'tsnode.rs'), 'w') as f:
        f.write(seed_line)
        f.write('\n')
        f.writelines('\n'.join(lines))
        f.write('\n')
        f.write(rust_hash_function())


def main():
    node_types_path = Path('tree-sitter-universql', 'src', 'node-types.json')

    node_types = []
    with open(node_types_path, 'r') as f:
        node_types = json.load(f)

    generate_nodes(node_types)
    

if __name__ == '__main__':
    main()
