package Day05;

use strict;
use warnings;
use v5.26;

sub new {
    my ($class, $input) = @_;

    my %self;
    $self{input} = $input;

    return bless(\%self, $class);
}

sub parse_stacks {
    my ($stacks) = @_;

    my @stack_inputs = grep { /\[/ } reverse split '\n', $stacks;
    my $stack_length = (length($stack_inputs[0]) + 1) / 4;
    my @stacks;
    push @stacks, [] for 1..$stack_length;
    for my $stack (@stack_inputs) {
        for my $i (0..$stack_length) {
            my $lookup_index = $i * 4 + 1;
            next if $lookup_index > length($stack);
            my $char = substr $stack, $lookup_index, 1;
            if ($char && $char =~ m/[A-Z]/) {
                push @{$stacks[$i]}, $char;
            }
        }
    }
    return \@stacks;
}

sub part1 {
    my ($self) = @_;
    my $input = $self->{input};

    my ($stacks, $procedures) = split '\n\n', $input;
    $stacks = parse_stacks($stacks);
    my @procedures = map { [ grep { /\d+/ } split ' ' ] } split '\n', $procedures;

    for my $procedure (@procedures) {
        my ($amount, $from, $to) = @{$procedure};
        do {
            my $item = pop @{$stacks->[$from  - 1]};
            push @{$stacks->[$to - 1]}, $item;
        } while --$amount > 0;
    }
    return join '', map { pop @$_ } @{$stacks};
}

sub part2 {
    my ($self) = @_;
    my $input = $self->{input};

    my ($stacks, $procedures) = split '\n\n', $input;
    $stacks = parse_stacks($stacks);
    my @procedures = map { [ grep { /\d+/ } split ' ' ] } split '\n', $procedures;

    for my $procedure (@procedures) {
        my ($amount, $from, $to) = @{$procedure};
        my @items = splice @{$stacks->[$from  - 1]}, -$amount;
        for (@items) {
            push @{$stacks->[$to - 1]}, $_;
        }
    }
    return join '', map { pop @$_ } @{$stacks};
}

1;
