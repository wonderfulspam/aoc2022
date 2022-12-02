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
            $score += 4 when $_->[0] eq "A" && $_->[1] eq "X"; # Rock + Rock = Draw
            $score += 8 when $_->[0] eq "A" && $_->[1] eq "Y"; # Rock + Paper = Win
            $score += 3 when $_->[0] eq "A" && $_->[1] eq "Z"; # Rock + Scissors = Loss
            $score += 1 when $_->[0] eq "B" && $_->[1] eq "X"; # Paper + Rock = Loss
            $score += 5 when $_->[0] eq "B" && $_->[1] eq "Y"; # Paper + Paper = Draw
            $score += 9 when $_->[0] eq "B" && $_->[1] eq "Z"; # Paper + Scissors = Win
            $score += 7 when $_->[0] eq "C" && $_->[1] eq "X"; # Scissors + Rock = Win
            $score += 2 when $_->[0] eq "C" && $_->[1] eq "Y"; # Scissors + Paper = Loss
            $score += 6 when $_->[0] eq "C" && $_->[1] eq "Z"; # Scissors + Scissors = Draw
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
