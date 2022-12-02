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

# You need a file containing a tab-separated line like this:
# adventofcode.com	FALSE	/	TRUE	<expiry_as_unix_timestamp>	session <cookie_value>
# Get puzzle input for a given day
get_input day:
  curl \
    --silent \
    --cookie "session_cookie.txt" \
    --output ./inputs/day{{day}} \
    https://adventofcode.com/2022/day/{{day}}/input
