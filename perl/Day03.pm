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
    return;
}

sub part2 {
    my ($self) = @_;
    my $input = $self->{input};
    return;
}

1;
