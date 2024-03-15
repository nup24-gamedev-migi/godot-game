package com.mygdx.game;

import java.util.*;
import java.util.stream.Stream;

public class Logic {
    // TODO things probably should be made into classes

    // TODO make it generic!!!!!
    public class Pair {
        public Pos pos;
        public MoveDirection dir;
        Pair(Pos pos, MoveDirection dir) {
            this.pos = pos;
            this.dir = dir;
        }
    }
    public enum CellState {
        VISITED,
        UNVISITED,
        CYCLE
    }
    public enum ThingType {
        PLAYER,
        BOX,
    }
    public enum MoveDirection {
        LEFT,
        RIGHT,
        UP,
        DOWN,
    }
    public enum CellType {
        FLOOR,
        WALL,
        ENTRANCE,
        TREASURE
    }
    public static class Cell {
        public CellType type;
        public boolean hasShadow;

        Cell (CellType type, boolean hasShadow) {
            this.type = type;
            this.hasShadow = hasShadow;
        }

        public String toShortString() {
            if (hasShadow) {
                return "X";
            }

            switch (type) {
                case FLOOR: return " ";
                case WALL: return "W";
                case ENTRANCE: return "E";
                case TREASURE: return "$";
            }

            return "?";
        }
    }

    public static class Pos {
        public int x;
        public int y;

        public Pos(int x, int y) {
            this.x = x;
            this.y = y;
        }

        public Pos applyDir(final MoveDirection dir) {
            switch (dir) {
                case LEFT:  return new Pos(x - 1, y);
                case RIGHT: return new Pos(x + 1, y);
                case UP:    return new Pos(x, y - 1);
                case DOWN:  return new Pos(x, y + 1);
            }

            return this;
        }

        @Override
        public String toString() {
            return "(" + x + ", " + y + ")";
        }

        @Override
        public int hashCode () {
            final int prime = 31;
            int result = 1;
            result = prime * result + x;
            result = prime * result + y;
            return result;
        }

        @Override
        public boolean equals (Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Pos other = (Pos)obj;
            if (x != other.x) return false;
            if (y != other.y) return false;
            return true;
        }
    }

    public int getFieldWidth() {
        return fieldWidth;
    }

    public int getFieldHeight() {
        return fieldHeight;
    }

    public Cell getCell(final int x, final int y) {
        return field[y][x];
    }

    private final Map<Pos, ThingType> thingTypeMap;
    private final Cell[][] field;
    private Pos playerPos;
    private final int fieldWidth;
    private final int fieldHeight;
    private final List<Pair> history;

    public Logic(final CellType[][] field, final Map<Pos, ThingType> thingTypeMap) {
        // TODO make this constructor argument
        history = new ArrayList<>();
        playerPos = thingTypeMap.entrySet()
                .stream()
                .filter(x -> x.getValue() == ThingType.PLAYER)
                .map(Map.Entry::getKey)
                .findFirst()
                .orElseThrow();
        fieldHeight = field.length;
        fieldWidth = field[0].length;
        this.thingTypeMap = new HashMap<>(thingTypeMap);

        this.field = new Cell[fieldHeight][fieldWidth];
        for (int y = 0; y < fieldHeight; y++) {
            for (int x = 0; x < fieldWidth; x++) {
                this.field[y][x] = new Cell(field[y][x], false);
            }
        }
    }

    public Collection<Pair> getHistory() {
        return Collections.unmodifiableCollection(history);
    }

    public void movePlayer(final MoveDirection dir) {
        if (moveThing(playerPos, dir)) {
            playerPos = playerPos.applyDir(dir);
            history.add(new Pair(playerPos, dir));
        }
    }

    // TODO should be private and called when treasure was stolen.
    public void applyShadowToField() {
        CellState[][] visited = new CellState[fieldHeight][fieldWidth];
        for (CellState[] row : visited) {
            Arrays.fill(row, CellState.UNVISITED);
        }

        for (int i = 0; i < history.size() - 1; i++) {
            Pos currentCellPos = history.get(i).pos;
//            getCell(currentCellPos.x, currentCellPos.y).hasShadow = true; TODO (move to another place!)
            if ( visited[currentCellPos.y][currentCellPos.x] == CellState.UNVISITED) {
                visited[currentCellPos.y][currentCellPos.x] = CellState.VISITED;
            } else if (visited[currentCellPos.y][currentCellPos.x] == CellState.VISITED) {
                int j = i - 1;
                Pos curPosToFindCycle = history.get(j).pos;

                for (;! curPosToFindCycle.equals(currentCellPos); j--) {
                    curPosToFindCycle = history.get(j).pos;
                    visited[curPosToFindCycle.y][curPosToFindCycle.x] = CellState.CYCLE;
                }
                if (! validateCycle(visited, currentCellPos, j)) {
                    int h = i - 1;
                    Pos curPos;
                    for (; h >= j; h--) {
                        curPos = history.get(h).pos;
                        if (visited[curPos.y][curPos.x] == CellState.CYCLE)
                            visited[curPos.y][curPos.x] = CellState.VISITED;
                    }
                }
            }
        }
        for (Pair record: history) {
            Pos curPos = record.pos;
            if (visited[curPos.y][curPos.x] == CellState.VISITED) {
                getCell(curPos.x, curPos.y).hasShadow = true;
            }
        }
    }

    private Pos findMostLeftPos(int historyIndexOfStartLower, Pos startCyclePos) {
        Pos currentLeftMin = startCyclePos;
        for (int i = historyIndexOfStartLower; history.get(i).pos != startCyclePos; i++) {
            Pos currentPos = history.get(i).pos;
            if (currentLeftMin.x > currentPos.x) {
                currentLeftMin = currentPos;
            }
        }
        return currentLeftMin;
    }

    private Pos findMostTopPos(int historyIndexOfStartLower, Pos startCyclePos) {
        Pos currentTopMin = startCyclePos;
        for (int i = historyIndexOfStartLower; history.get(i).pos != startCyclePos; i++) {
            Pos currentPos = history.get(i).pos;
            if (currentTopMin.y > currentPos.y) {
                currentTopMin = currentPos;
            }
        }
        return currentTopMin;
    }

    private int findRightMost(int historyIndexOfStartLower, Pos startCyclePos) {
        int currentRightMax = startCyclePos.x;
        for (int i = historyIndexOfStartLower; history.get(i).pos != startCyclePos; i++) {
            Pos currentPos = history.get(i).pos;
            if (currentRightMax < currentPos.x) {
                currentRightMax = currentPos.x;
            }
        }
        return currentRightMax;
    }

    private boolean validateCycle( CellState[][] visited, Pos startCyclePos, int historyIndexOfStartLower) {
        Pos mostLeftPos = findMostLeftPos(historyIndexOfStartLower, startCyclePos);
        Pos mostTopPos = findMostTopPos(historyIndexOfStartLower, startCyclePos);
        int rightMostX = findRightMost(historyIndexOfStartLower, startCyclePos);
        Pos currentPos = new Pos(mostLeftPos.x, mostTopPos.y);

        for (;currentPos.y < fieldHeight; currentPos.y++){
            boolean stepIntoCycle = false;
            currentPos.x = mostLeftPos.x;
            for (; currentPos.x < fieldWidth && currentPos.x <= rightMostX; currentPos.x++) {
                if (visited[currentPos.y][currentPos.x] == CellState.UNVISITED) {
                    if (stepIntoCycle) {
                        return true;
                    } else {
                        continue;
                    }
                }
                if (visited[currentPos.y][currentPos.x] == CellState.VISITED) {
                    continue;
                }
                if (visited[currentPos.y][currentPos.x] == CellState.CYCLE) {
                    if (! stepIntoCycle) {
                        if (currentPos.x + 1 <= rightMostX && visited[currentPos.y][currentPos.x + 1] != CellState.CYCLE) {
                            stepIntoCycle = true;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        return false;
    }

    public Stream<Map.Entry<Pos, ThingType>> allThings() {
        return thingTypeMap.entrySet().stream();
    }

    private boolean moveThing(final Pos thingPos, final MoveDirection dir) {
        final Pos newThingPos = thingPos.applyDir(dir);

        if (!thingTypeMap.containsKey(thingPos)) {
            return true; // Might want to return false
        }

        if (!posValid(newThingPos)) {
            return false;
        }

        if (!thingTypeMap.containsKey(newThingPos)) {
            thingTypeMap.put(newThingPos, thingTypeMap.remove(thingPos));
            return true;
        }

        /* Obstacle at newPos. Let's try to push it away */
        final Pos obstacleNewPos = newThingPos.applyDir(dir);
        if (!posValid(obstacleNewPos)) {
            return false;
        }
        if (thingTypeMap.containsKey(obstacleNewPos)) {
            return false;
        }

        thingTypeMap.put(obstacleNewPos, thingTypeMap.remove(newThingPos));
        thingTypeMap.put(newThingPos, thingTypeMap.remove(thingPos));

        return true;
    }

    /* Function checks that you can go or slide a box on this position. */
    private boolean posValid(final Pos pos) {
        if (pos.x < 0 || pos.y < 0) {
            return false;
        }

        if (pos.x >= fieldWidth || pos.y >= fieldHeight) {
            return false;
        }

        final Cell cell = getCell(pos.x, pos.y);

        if (cell.type == CellType.WALL) {
            return false;
        }

        return !cell.hasShadow;
    }
}
