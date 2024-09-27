# Teste 1

Duration: 267.83381857s
Jumped lines: 0
Min/Mean/Max: -69.1/64.25722/88.3
Bin: slow
Config: Cargo.toml conf padrão, --release flag

Just getting most basic way of doing the min/max/mean without grouping by station

---

# Teste 2

Duration: 257.819333275s
Jumped lines: 0
Min/Mean/Max: -69.1/64.25722/88.
Bin: slow
Config: Cargo.toml conf ligeira, --release flag, target-cpu=native flag

Same as test 1 but with more nice config for compiler

---

# Teste 3

Duration: 16.610718838s

Grouping by station and using parallelism and some tricks with integer
Need to order list
