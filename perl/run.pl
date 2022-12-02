#!/usr/bin/env perl

use strict;
use warnings;
use v5.26;

use File::Basename;
use lib dirname (__FILE__);
use Getopt::Long;
use File::Slurper qw(read_text);
use FindBin qw($Bin);

# Parse --day input
my $day;
GetOptions(
  'day=i' => \$day,
) or die 'Invalid input';

# Read input file (dies on error)
my $input = read_text("$Bin/../inputs/day$day");

# Load relevant module
my $module_name = sprintf "Day%02d", $day;
eval "require $module_name" or die $@;

# Instantiate with input
my $module = $module_name->new($input);

# Run and print
my $part1 = $module->part1;
my $part2 = $module->part2;
say "Part 1: $part1";
say "Part 2: $part2";
