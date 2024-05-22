package com.mygdx.game;

import java.util.*;
import java.util.stream.Collectors;
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

    Cell(CellType type, boolean hasShadow) {
      this.type = type;
      this.hasShadow = hasShadow;
    }

    public String toShortString() {
      if (hasShadow) {
        return "X";
      }

      switch (type) {
        case FLOOR:
          return " ";
        case WALL:
          return "W";
        case ENTRANCE:
          return "E";
        case TREASURE:
          return "$";
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
        case LEFT:
          return new Pos(x - 1, y);
        case RIGHT:
          return new Pos(x + 1, y);
        case UP:
          return new Pos(x, y - 1);
        case DOWN:
          return new Pos(x, y + 1);
      }

      return this;
    }

    @Override
    public String toString() {
      return "(" + x + ", " + y + ")";
    }

    @Override
    public int hashCode() {
      final int prime = 31;
      int result = 1;
      result = prime * result + x;
      result = prime * result + y;
      return result;
    }

    @Override
    public boolean equals(Object obj) {
      if (this == obj) return true;
      if (obj == null) return false;
      if (getClass() != obj.getClass()) return false;
      Pos other = (Pos) obj;
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
    if (x < 0) {
      return null;
    }
    if (y < 0) {
      return null;
    }
    if (x >= fieldWidth) {
      return null;
    }
    if (y >= fieldHeight) {
      return null;
    }
    return field[y][x];
  }

  private final Map<Pos, ThingType> thingTypeMap;
  private final Cell[][] field;
  private Pos playerPos;
  private final int fieldWidth;
  private final int fieldHeight;
  private final List<Pair> history;
  private boolean isTreasureStolen; // update this when loading new level

  private static boolean doBoxDrop = true;

  public Logic(final CellType[][] field, final Map<Pos, ThingType> thingTypeMap) {
    history = new ArrayList<>();
    playerPos = findPlayerPos(field);
    fieldHeight = field.length;
    fieldWidth = field[0].length;

    this.thingTypeMap = new HashMap<>(thingTypeMap
            .entrySet().stream()
            .filter(x -> x.getValue() != ThingType.PLAYER)
            .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue))
    );
    this.thingTypeMap.put(playerPos, ThingType.PLAYER);

    this.field = new Cell[fieldHeight][fieldWidth];
    for (int y = 0; y < fieldHeight; y++) {
      for (int x = 0; x < fieldWidth; x++) {
        this.field[y][x] = new Cell(field[y][x], false);
      }
    }
    isTreasureStolen = false;
  }

  private Pos findPlayerPos(CellType[][] field) {
    Pos player = new Pos(0, 0);
    for (int i = 0; i < field.length; i++) {
      for (int j = 0; j < field[i].length; j++) {
        if (field[i][j] == CellType.ENTRANCE) {
          player = new Pos(j, i);
        }
      }
    }
    return player;
  }

  public List<Pair> getHistory() {
    return Collections.unmodifiableList(history);
  }

  public boolean isTreasureStolen() {
    return isTreasureStolen;
  }

  public boolean isPlayerAtEntrace() {
    return getCell(playerPos.x, playerPos.y).type == CellType.ENTRANCE;
  }


  public void movePlayer(final MoveDirection dir) {
    if (moveThing(playerPos, dir)) {
      playerPos = playerPos.applyDir(dir);
      history.add(new Pair(playerPos, dir));
      if (getCell(playerPos.x, playerPos.y).type == CellType.TREASURE && !isTreasureStolen) {
        applyShadowToField();
      }
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
      if (visited[currentCellPos.y][currentCellPos.x] == CellState.UNVISITED) {
        visited[currentCellPos.y][currentCellPos.x] = CellState.VISITED;
      } else if (visited[currentCellPos.y][currentCellPos.x] == CellState.VISITED) {
        int j = i - 1;
        Pos curPosToFindCycle = history.get(j).pos;

        for (; !curPosToFindCycle.equals(currentCellPos); j--) {
          curPosToFindCycle = history.get(j).pos;
          visited[curPosToFindCycle.y][curPosToFindCycle.x] = CellState.CYCLE;
        }
        if (!validateSimpleCycle(visited, currentCellPos, ++j, i)) {
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
    isTreasureStolen = true;

    for (Pair record : history) {
      Pos curPos = record.pos;
      if (visited[curPos.y][curPos.x] == CellState.VISITED) {
        getCell(curPos.x, curPos.y).hasShadow = true;
      }
    }
  }

  private int findMostLeftX(int historyIndexOfStartLower, Pos startCyclePos) {
    Pos currentLeftMin = startCyclePos;
    for (int i = historyIndexOfStartLower; history.get(i).pos != startCyclePos; i++) {
      Pos currentPos = history.get(i).pos;
      if (currentLeftMin.x > currentPos.x) {
        currentLeftMin = currentPos;
      }
    }
    return currentLeftMin.x;
  }

  private int findMostTopY(int historyIndexOfStartLower, Pos startCyclePos) {
    Pos currentTopMin = startCyclePos;
    for (int i = historyIndexOfStartLower; history.get(i).pos != startCyclePos; i++) {
      Pos currentPos = history.get(i).pos;
      if (currentTopMin.y > currentPos.y) {
        currentTopMin = currentPos;
      }
    }
    return currentTopMin.y;
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

  private boolean validateCycle(CellState[][] visited, Pos startCyclePos, int historyIndexOfStartLower) {
    int mostLeftX = findMostLeftX(historyIndexOfStartLower, startCyclePos);
    int mostTopY = findMostTopY(historyIndexOfStartLower, startCyclePos);
    int rightMostX = findRightMost(historyIndexOfStartLower, startCyclePos);
    Pos currentPos = new Pos(mostLeftX, mostTopY);

    for (; currentPos.y < fieldHeight; currentPos.y++) {
      boolean stepIntoCycle = false;
      currentPos.x = mostLeftX;
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
          if (!stepIntoCycle) {
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

  private boolean checkIfCycleIsRectangle(int historyIndexOfStartLower,
                                          int historyIndexOfEndUpper) {
    int numberOfChangeDir = 0;
    // There we count a number of change directions on the cycle.
    // If this number is bigger than 5 - this is not a valid cycle.
    for (int i = historyIndexOfStartLower + 1; i <= historyIndexOfEndUpper; i++) {
      if (! history.get(i - 1).dir.equals( history.get(i).dir)) {
        numberOfChangeDir++;
      }
    }
    return numberOfChangeDir <= 5;
  }
  private boolean checkIfCycleIsBigEnough(CellState[][] visited, Pos startCyclePos, int historyIndexOfStartLower,
                                          int historyIndexOfEndUpper) {
    Pos leftTopCorner = new Pos(findMostLeftX(historyIndexOfStartLower, startCyclePos),
            findMostTopY(historyIndexOfStartLower, startCyclePos));
    Pos rightTopCorner = new Pos(findRightMost(historyIndexOfStartLower, startCyclePos), findMostTopY(historyIndexOfStartLower, startCyclePos));

    return rightTopCorner.x - leftTopCorner.x > 1;
  }

  private boolean validateSimpleCycle(CellState[][] visited, Pos startCyclePos, int historyIndexOfStartLower,
                                      int historyIndexOfEndUpper) {
    // simple check that it is not rectangle
    if ((historyIndexOfEndUpper - historyIndexOfStartLower) % 2 == 1) {
      return false;
    }
    // check that it is not rectangle
    if (! checkIfCycleIsRectangle(historyIndexOfStartLower, historyIndexOfEndUpper)) {
      return false;
    }
    // check that there is at list one not visited cell at the middle
    return checkIfCycleIsBigEnough(visited, startCyclePos, historyIndexOfStartLower, historyIndexOfEndUpper);
  }

  public Stream<Map.Entry<Pos, ThingType>> allThings() {
    return thingTypeMap.entrySet().stream();
  }

  private boolean moveThing(final Pos thingPos, final MoveDirection dir) {
    final Pos newThingPos = thingPos.applyDir(dir);

    if (!thingTypeMap.containsKey(thingPos)) {
      return true; // Might want to return false
    }

    if (!posValid(newThingPos, thingTypeMap.get(thingPos))) {
      return false;
    }

    if (!thingTypeMap.containsKey(newThingPos)) {
      thingTypeMap.put(newThingPos, thingTypeMap.remove(thingPos));
      return true;
    }

    /* Obstacle at newPos. Let's try to push it away */
    final Pos obstacleNewPos = newThingPos.applyDir(dir);
    if (!posValid(obstacleNewPos, thingTypeMap.get(newThingPos))) {
      return false;
    }
    if (thingTypeMap.containsKey(obstacleNewPos)) {
      return false;
    }

    final ThingType obstacle = thingTypeMap.remove(newThingPos);
    if (getCell(obstacleNewPos.x, obstacleNewPos.y).type != CellType.WALL) {
      thingTypeMap.put(obstacleNewPos, obstacle);
    } else {
      getCell(obstacleNewPos.x, obstacleNewPos.y).type = CellType.FLOOR;
    }
    thingTypeMap.put(newThingPos, thingTypeMap.remove(thingPos));

    return true;
  }

  /* Function checks that you can go or slide a box on this position. */
  private boolean posValid(final Pos pos, final ThingType ty) {
    if (pos.x < 0 || pos.y < 0) {
      return false;
    }

    if (pos.x >= fieldWidth || pos.y >= fieldHeight) {
      return false;
    }

    final Cell cell = getCell(pos.x, pos.y);

    if (cell.type == CellType.WALL) {
      if (ty == null) {
        return false;
      }
      return ty == ThingType.BOX && doBoxDrop;
    }

    return !cell.hasShadow;
  }
}
