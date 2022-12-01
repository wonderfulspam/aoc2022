#!/usr/bin/env perl

use strict;
use warnings;
use v5.26;

use File::Slurper qw(read_text);
use FindBin qw($Bin);
use List::Util qw(sum);

my $input = read_text("$Bin/../inputs/day1");

my @counts = sort { $b <=> $a } map { sum(split '\n') } split '\n\n', $input;

my $max = $counts[0];
my $max_three = sum(@counts[0..2]);

say "Max: $max";
say "Max three: $max_three";
