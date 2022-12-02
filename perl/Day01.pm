package Day01;

use strict;
use warnings;
use v5.26;

use List::Util qw(sum);


sub new {
    my ($class, $input) = @_;

    my %self;
    my @counts = sort { $b <=> $a } map { sum(split '\n') } split '\n\n', $input;
    $self{counts} = \@counts;

    return bless(\%self, $class);
}

sub part1 {
    my ($self) = @_;
    return $self->{counts}->[0];
}

sub part2 {
    my ($self) = @_;
    return sum(@{$self->{counts}}[0..2]);
}

1;
