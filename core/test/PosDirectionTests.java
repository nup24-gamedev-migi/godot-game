import com.mygdx.game.Logic;
import org.junit.jupiter.api.RepeatedTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Assertions;

import java.util.Random;

public class PosDirectionTests {
    @Test
    public void testSimpleApplyDirection() {
        // TODO better coverage
        final Logic.Pos pos = new Logic.Pos(0, 0);
        final Logic.Pos exPos = new Logic.Pos(-1, 0);

        Assertions.assertEquals(exPos, pos.applyDir(Logic.MoveDirection.LEFT));
    }

    @Test
    public void testDirectionCircle() {
        Logic.Pos pos = new Logic.Pos(0, 0);
        final Logic.Pos expectPos = new Logic.Pos(0, 0);

        pos = pos.applyDir(Logic.MoveDirection.RIGHT);
        pos = pos.applyDir(Logic.MoveDirection.DOWN);
        pos = pos.applyDir(Logic.MoveDirection.LEFT);
        pos = pos.applyDir(Logic.MoveDirection.UP);

        Assertions.assertEquals(expectPos, pos);
    }

    @RepeatedTest(5)
    public void testDirectionHugeRandom() {
        Logic.Pos pos = new Logic.Pos(0, 0);
        final Logic.Pos expectPos = new Logic.Pos(0, 0);
        Random rnd = new Random();

        for (int i = 0; i < rnd.nextInt(1000, 10000); i++) {
            pos = pos.applyDir(Logic.MoveDirection.RIGHT);
            pos = pos.applyDir(Logic.MoveDirection.DOWN);
            pos = pos.applyDir(Logic.MoveDirection.LEFT);
            pos = pos.applyDir(Logic.MoveDirection.UP);
        }

        Assertions.assertEquals(expectPos, pos);
    }

}
