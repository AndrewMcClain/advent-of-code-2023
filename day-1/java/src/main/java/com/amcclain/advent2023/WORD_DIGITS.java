package com.amcclain.advent2023;

public enum WORD_DIGITS {
    ONE("one",'1'),
    TWO("two", '2'),
    THREE("three", '3'),
    FOUR("four", '4'),
    FIVE("five", '5'),
    SIX("six", '6'),
    SEVEN("seven", '7'),
    EIGHT("eight", '8'),
    NINE("nine", '9');

    final String text;
    final char value;
    WORD_DIGITS(String word, char value) {
        this.text = word;
        this.value = value;
    }
}
