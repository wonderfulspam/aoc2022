package Day03;

use strict;
use warnings;
use v5.26;

sub new {
    my ($class, $input) = @_;

    my %self;
    $self{input} = $input;

    return bless(\%self, $class);
}

sub part1 {
    my ($self) = @_;
    my $input = $self->{input};

    my $sum = 0;
    for my $line (split '\n', $input) {
        my $len = length($line) / 2;
        my $first_part = substr $line, 0, $len;
        my $second_part = substr $line, $len, length($line);
        my @common = common_chars($first_part, $second_part);
        $sum += char_to_value($common[0])
    }
    return $sum;
}

sub part2 {
    my ($self) = @_;
    my $input = $self->{input};

    my $sum = 0;
    my @lines = split '\n', $input;
    while(my @chunks = splice(@lines, 0, 3)) {
        my @common = common_chars($chunks[0], $chunks[1]);
        @common = common_chars("@common", $chunks[2]);
        $sum += char_to_value($common[0])
    }
    return $sum;
}

sub common_chars {
    my ($str1, $str2) = @_;
    my %u;
    $u{ $_ } = 1 for split //, $str1;
    return grep --$u{ $_ } >= 0, split //, $str2;
}

sub char_to_value {
    my $char = shift;
    if ($char =~ /[a-z]/) {
        return ord($char) - ord('a') + 1;
    }
    if ($char =~ /[A-Z]/) {
        return ord($char) - ord('A') + 27;
    }
    die 'Unreachable';
}

1;
