# Como gerar os stubs PHP da extensão (para o PhpStorm)

Guia rápido para regenerar os stubs `.php` da extensão `extensao_ds`
(escrita em Rust com `ext-php-rs`). Os stubs servem só para o IDE
(autocomplete, tipos, "undefined class") — não afetam a execução.

---

## Setup (só na primeira vez, ou em máquina nova)

O `cargo php stubs` quebra com `undefined symbol: zend_hash_del` porque o
`cargo-php` precisa dos símbolos internos do PHP, que só existem numa lib
compartilhada (o "embed SAPI"). Corrige-se em 2 passos:

```bash
# 1. Instala o embed SAPI -> cria /lib/libphp.so (exporta os símbolos do PHP).
#    Obs: NÃO existe pacote "php8.4-embed"; o embed vem só no libphp8.4-embed.
sudo apt install -y libphp8.4-embed

# 2. Recompila o cargo-php FORÇANDO o link contra a libphp.
#    Um "cargo install cargo-php --force" normal NÃO linka a libphp — precisa dos RUSTFLAGS.
#    -L /lib            -> onde achar a libphp.so
#    -C link-arg=-lphp  -> adiciona -lphp ao link (cria o DT_NEEDED)
RUSTFLAGS="-L /lib -C link-arg=-lphp" cargo install cargo-php --force
```

Pré-requisitos do toolchain (já costumam estar instalados):
`php8.4-dev` (headers do PHP), `clang` + `libclang-dev` (bindgen), `rustc`/`cargo`.

Conferir se o link funcionou:

```bash
ldd ~/.cargo/bin/cargo-php | grep php     # deve mostrar: libphp.so => /lib/libphp.so
```

---

## Gerar os stubs (sempre que a extensão mudar)

O `cargo-php` NÃO cria a pasta sozinho — por isso o `mkdir -p` antes.

```bash
cd /home/cascata/extensao_varinha
mkdir -p stubs
cargo php stubs --out stubs/extensao_ds.stubs.php
```

Ou, via cargo-make (task já definida no Makefile.toml):

```bash
cargo make stubs
```

---

## Fluxo completo ao mexer na extensão

```bash
cd /home/cascata/extensao_varinha
cargo make deploy   # recompila e instala o .so no PHP (precisa de sudo)
mkdir -p stubs
cargo make stubs    # regenera o stub .php
```

Depois, no PhpStorm: **File → Reload All from Disk** (ele não percebe
arquivos criados por fora, via WSL/terminal).

---

## Resumo de 1 linha

`libphp8.4-embed` → reinstalar `cargo-php` com `RUSTFLAGS` → `mkdir -p stubs` → `cargo php stubs`.
