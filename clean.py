import toml

with open("Cargo.toml", "r") as file:
    cargo_toml = toml.load(file)

bins = cargo_toml.get('bin', [])
bins = [dict(t) for t in set([tuple(d.items()) for d in bins])]
bins.sort(key = lambda x: x['name'])

cargo_toml['bin'] = bins

with open("Cargo.toml", "w") as file:
    toml.dump(cargo_toml, file)