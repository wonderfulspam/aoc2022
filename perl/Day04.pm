package Day04;

use strict;
use warnings;
use v5.26;

sub new {
    my ($class, $input) = @_;

    my %self;

    my @pairs =  map { [ ((split '-', $_->[0]), (split '-', $_->[1])) ] } map { [ split ',' ] }  split '\n', $input;
    $self{pairs} = \@pairs;
    return bless(\%self, $class);
}

sub part1 {
    my ($self) = @_;
    my $pairs = $self->{pairs};

    my $count = grep { ($_->[0] >= $_->[2] && $_->[1] <= $_->[3]) || ($_->[2] >= $_->[0] && $_->[3] <= $_->[1]) } @{$pairs};
    return $count;
}

sub part2 {
    my ($self) = @_;
    my $pairs = $self->{pairs};

    my $count = grep { $_->[2] <= $_->[1] && $_->[0] <= $_->[3] } @{$pairs};
    return $count;
}

1;
