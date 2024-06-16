import com.mygdx.game.Logic;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.function.Consumer;

public class TreasureGrabTests {

    int treasureCounter = 0;
    private Logic.MoveDirection[] parseMoveSequence(final String code) {
        final char[] chars = code.toCharArray();
        final Logic.MoveDirection[] dirs = new Logic.MoveDirection[code.length()];

        for (int i = 0; i < dirs.length; i++) {
            switch (chars[i]) {
                case 'U':
                    dirs[i] = Logic.MoveDirection.UP;
                    break;
                case 'L':
                    dirs[i] = Logic.MoveDirection.LEFT;
                    break;
                case 'D':
                    dirs[i] = Logic.MoveDirection.DOWN;
                    break;
                case 'R':
                    dirs[i] = Logic.MoveDirection.RIGHT;
                    break;
                default:
                    throw new IllegalArgumentException(
                            String.format("Illegal char '%c' at %d", chars[i], i)
                    );
            }
        }

        return dirs;
    }

    public Logic.Pos searchPos(Logic logic, Logic.CellType obj) {
        Logic.Pos pos = new Logic.Pos(0, 0);

        for (int y = 0; y < logic.getFieldHeight(); y++) {
            for (int x = 0; x < logic.getFieldWidth(); x++) {
                if (logic.getCell(x, y).type == obj) {
                    pos.x = x;
                    pos.y = y;
                }
            }
        }
        return pos;
    }

    private String encodeShadow(final Logic logic) {
        StringBuilder acc = new StringBuilder();

        for (int y = 0; y < logic.getFieldHeight(); y++) {
            for (int x = 0; x < logic.getFieldWidth(); x++) {
                if (logic.getCell(x, y).hasShadow) {
                    acc.append("x");
                } else if (logic.getCell(x, y).type == Logic.CellType.TREASURE) {
                    acc.append("$");
                } else {
                    acc.append("o");
                }
            }
            acc.append("\n");
        }
        return acc.toString();
    }

    private Logic.CellType[][] genEmptyField(final int width, final int height) {
        final Logic.CellType[][] field = new Logic.CellType[height][width];

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                field[y][x] = Logic.CellType.FLOOR;
            }
        }
        return field;
    }

    private boolean[][] shadowCoverage(final Logic logic, final int width, final int height) {
        boolean[][] shadowField = new boolean[height][width];

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                shadowField[y][x] = logic.getCell(x, y).hasShadow;
            }
        }
        return shadowField;
    }

    private void tryMoves(
            final Logic logic,
            final Logic.MoveDirection[] moves,
            final String shadowExpectation
    ) {

        Arrays.stream(moves).forEach(logic::movePlayer);
//        logic.applyShadowToField();

        Assertions.assertEquals(
                "\n" + shadowExpectation.trim(),
                "\n" + encodeShadow(logic).trim()
        );
    }

    private void tryMoves(
            final Logic logic,
            final String movesCode,
            final String shadowExpectation
    ) {
        final Logic.MoveDirection[] moves = parseMoveSequence(movesCode);

        tryMoves(logic, moves, shadowExpectation);
    }

    private Logic tryMoves(
            final int fieldWidth,
            final int fieldHeight,
            final Logic.Pos playerPos,
            final Logic.Pos treasurePos,
            final String movesCode,
            final String shadowExpectation
    ) {
        final Logic.CellType[][] field = genEmptyField(fieldWidth, fieldHeight);
        field[playerPos.y][playerPos.x] = Logic.CellType.ENTRANCE;
        field[treasurePos.y][treasurePos.x] = Logic.CellType.TREASURE;
        final Logic logic = new Logic(field, new HashMap<>());

        makeAssertion(logic, fieldWidth, fieldHeight);

        tryMoves(logic, movesCode, shadowExpectation);
        return logic;
    }

    private void makeAssertion(
            Logic logic,
            final int fieldWidth,
            final int fieldHeight
    ) {
        Assertions.assertNotEquals(fieldHeight, 0);
        Assertions.assertNotEquals(fieldWidth, 0);

        Assertions.assertEquals(1, logic.allThings().filter(x -> x.getValue() == Logic.ThingType.PLAYER).count());
        Logic.Pos act = logic.allThings().filter(x -> x.getValue() == Logic.ThingType.PLAYER).findFirst().get().getKey();
        Logic.Pos exp = searchPos(logic, Logic.CellType.ENTRANCE);
        Assertions.assertEquals(exp, act);
    }

    @Test
    public void testCoverSimple() {
        Logic logic = tryMoves(
                8, 8,
                new Logic.Pos(1, 1),
                new Logic.Pos(3, 3),
                "RRDD",
                "oooooooo\nooxxoooo\noooxoooo\nooo$oooo\noooooooo\noooooooo\noooooooo\noooooooo\n"
        );
        boolean[][] shadow = shadowCoverage(logic, 8, 8);

        tryMoves(
                logic,
                "DDUU",
                "oooooooo\nooxxoooo\noooxoooo\nooo$oooo\noooooooo\noooooooo\noooooooo\noooooooo\n"
        );
        boolean[][] newShadow = shadowCoverage(logic, 8, 8);

        Assertions.assertTrue(Arrays.deepEquals(shadow, newShadow));
    }

}
