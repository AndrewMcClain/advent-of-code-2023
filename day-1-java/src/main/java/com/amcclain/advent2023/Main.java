package com.amcclain.advent2023;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.concurrent.*;
import java.util.stream.Stream;

import static java.lang.Character.isDigit;
import static java.util.Arrays.*;

public class Main {
    public static void main(String[] args) throws IOException {
        Stream<String> lines = Files.lines(Path.of("input.txt"));

        ExecutorService executor = Executors.newFixedThreadPool(10);

        Set<Future<Integer>> sumFutures = new HashSet<>();
        lines.forEach((String s) -> {
            sumFutures.add(executor.submit(new LineSumFinder(s)));
        });
        int calibration = 0;
        for(Future<Integer> f : sumFutures) {
           try{
               calibration += f.get();
           } catch (Exception e) {}
        }
        executor.shutdown();
        System.out.println("Calibration Value is " + calibration);
    }


    static class LineSumFinder implements Callable<Integer> {
        private final String line;
        public LineSumFinder(String line) {
            this.line = line;
        }

        @Override
        public Integer call() throws Exception {
            Optional<Character> first = findFromStart(line);
            Optional<Character> second = findFromEnd(line);

            StringBuffer sb = new StringBuffer();
            sb.append('0');
            if(first.isPresent() && second.isPresent()) {
                sb.append(first.get());
                sb.append(second.get());
            }
            return Integer.valueOf(sb.toString());
        }

        private Optional<Character> findFromStart(String line) {
            for(int i=0; i < line.length(); i++) {
                Optional<Character> opt = getValueOfChar(line, i);
                if(opt.isPresent()) {
                    return opt;
                }
            }
            return Optional.empty();
        }

        private Optional<Character> findFromEnd(String line) {
            for(int i=line.length()-1; i >= 0; i--) {
                Optional<Character> opt = getValueOfChar(line, i);
                if(opt.isPresent()) {
                    return opt;
                }
            }
            return Optional.empty();
        }

        private Optional<Character> getValueOfChar(String line, int index) {
            char c = line.charAt(index);
            if(isDigit(c)) {
                return Optional.of(c);
            } else {
                Iterator<WORD_DIGITS> iter = stream(WORD_DIGITS.values()).iterator();
                while(iter.hasNext()) {
                    WORD_DIGITS d = iter.next();
                    int endingIndex = index+d.text.length();
                    if(endingIndex <= line.length()) {
                        String sub = line.substring(index, endingIndex);
                        if (d.text.equals(sub)) {
                            return Optional.of(d.value);
                        }
                    }
                }
            }
            return Optional.empty();
        }
    }
}

