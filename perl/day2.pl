#!/usr/bin/env perl

use strict;
use warnings;
use v5.26;

use File::Slurper qw(read_text);
use FindBin qw($Bin);

my $input = read_text("$Bin/../inputs/day2");
my $part1 = part1();
say "Part 1: $part1";
my $part2 = part2();
say "Part 2: $part2";

sub part1 {
    my $score = 0;
    for my $line (split '\n', $input) {
        my @choices = split ' ', $line;
        no warnings 'experimental';
        given (\@choices) {
            $score += 4 when $_->[1] eq "X" && $_->[0] eq "A"; # Rock + Rock = Draw
            $score += 1 when $_->[1] eq "X" && $_->[0] eq "B"; # Rock + Paper = Loss
            $score += 7 when $_->[1] eq "X" && $_->[0] eq "C"; # Rock + Scissors = Win
            $score += 8 when $_->[1] eq "Y" && $_->[0] eq "A"; # Paper + Rock = Win
            $score += 5 when $_->[1] eq "Y" && $_->[0] eq "B"; # Paper + Paper = Draw
            $score += 2 when $_->[1] eq "Y" && $_->[0] eq "C"; # Paper + Scissors = Loss
            $score += 3 when $_->[1] eq "Z" && $_->[0] eq "A"; # Scissors + Rock = Loss
            $score += 9 when $_->[1] eq "Z" && $_->[0] eq "B"; # Scissors + Paper = Win
            $score += 6 when $_->[1] eq "Z" && $_->[0] eq "C"; # Scissors + Scissors = Draw
        }
        use warnings;
    }
    return $score;
}

sub part2 {
    my $score = 0;
    for my $line (split '\n', $input) {
        my @choices = split ' ', $line;
        no warnings 'experimental';
        given (\@choices) {
            $score += 3 when $_->[0] eq "A" && $_->[1] eq "X"; # Rock + Loss = Scissors
            $score += 4 when $_->[0] eq "A" && $_->[1] eq "Y"; # Rock + Draw = Rock
            $score += 8 when $_->[0] eq "A" && $_->[1] eq "Z"; # Rock + Win = Paper
            $score += 1 when $_->[0] eq "B" && $_->[1] eq "X"; # Paper + Loss = Rock
            $score += 5 when $_->[0] eq "B" && $_->[1] eq "Y"; # Paper + Draw = Paper
            $score += 9 when $_->[0] eq "B" && $_->[1] eq "Z"; # Paper + Win = Scissors
            $score += 2 when $_->[0] eq "C" && $_->[1] eq "X"; # Scissors + Loss = Paper
            $score += 6 when $_->[0] eq "C" && $_->[1] eq "Y"; # Scissors + Draw = Scissors
            $score += 7 when $_->[0] eq "C" && $_->[1] eq "Z"; # Scissors + Win = Rock
        }
        use warnings;
    }
    return $score;
}
