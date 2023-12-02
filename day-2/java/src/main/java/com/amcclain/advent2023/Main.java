package com.amcclain.advent2023;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.*;
import java.util.stream.Stream;

public class Main {

    private static final int RED_CUBES = 12;
    private static final int GREEN_CUBES = 13;
    private static final int BLUE_CUBES = 14;

    public static void main(String[] args) throws IOException, ExecutionException, InterruptedException {
        Stream<String> lines = Files.lines(Path.of("input.txt"));

        ExecutorService executor = Executors.newFixedThreadPool(10);
        Set<Future<Integer>> futures = new HashSet<>();
        lines.forEach((String s) -> {
            futures.add(executor.submit(new LinePowerFinder(s)));
        });

        int validGamesSum = 0;
        for(Future<Integer> f : futures) {
            validGamesSum += f.get();
        }
        executor.shutdown();
        System.out.println("Sum of each games power is : " + validGamesSum);
    }

    static class LinePowerFinder implements Callable<Integer> {

        private final String line;
        private int minRed, minGreen, minBlue = 0;

        public LinePowerFinder(String line) {
            this.line = line;
        }

        @Override
        public Integer call() throws Exception {
            int gameIdEndIndex = line.indexOf(':');


            String[] rounds = line.substring(gameIdEndIndex+2).split(";");

            for(String round: rounds) {
                getMinCubes(round);
            }

            return minRed*minGreen*minBlue;
        }

        private void getMinCubes(String round) {
            String[] cubes = round.split(",");
            for(String s : cubes) {
                String cleanS = s.strip();
                String[] cubePull = cleanS.split(" ");
                int count = Integer.valueOf(cubePull[0]);
                switch(cubePull[1]) {
                    case "red":
                        minRed = Math.max(count, minRed);
                        break;
                    case "green":
                        minGreen = Math.max(count, minGreen);
                        break;
                    case "blue":
                        minBlue = Math.max(count, minBlue);
                        break;
                }
            }
        }
    }

    static class LineValidator implements Callable<Integer> {

        private final String line;
        private final int redCubes, greenCubes, blueCubes;

        public LineValidator(String line, int redCubes, int greenCubes, int blueCubes) {
            this.line = line;
            this.redCubes = redCubes;
            this.greenCubes = greenCubes;
            this.blueCubes = blueCubes;
        }

        @Override
        public Integer call() throws Exception {
            int gameIdStartIndex = line.indexOf(' ') + 1;
            int gameIdEndIndex = line.indexOf(':');
            int id = Integer.valueOf(line.substring(gameIdStartIndex, gameIdEndIndex));


            String[] rounds = line.substring(gameIdEndIndex+2).split(";");
            for(String round : rounds) {
                if(!isRoundValid(round)) {
                    return 0;
                }
            }
            return id;
        }

        private boolean isRoundValid(String round){
            boolean validRound = true;
            String[] cubes = round.split(",");
            for(String s : cubes) {
                String cleanS = s.strip();
                String[] cubePull = cleanS.split(" ");
                int limit = switch (cubePull[1]) {
                    case "red" -> redCubes;
                    case "green" -> greenCubes;
                    case "blue" -> blueCubes;
                    default -> 0;
                };
                if (Integer.valueOf(cubePull[0]) > limit) {
                    validRound = false;
                    break;
                }
            }
            return validRound;
        }
    }
}