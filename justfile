# Run perl solution for a given day
perl day:
  ./perl/run.pl --day {{day}}

# Run Rust solution for a given day
rust day:
  cargo run --quiet {{day}}

# Run Rust benchmarks
bench_rust:
  cargo bench

# Install perl dependencies
install_perl_deps:
  cpanm --sudo --installdeps .
