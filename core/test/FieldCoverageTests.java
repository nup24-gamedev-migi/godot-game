import com.mygdx.game.Logic;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

public class FieldCoverageTests {
    @Test
    public void testCover() {
        int testheight = 8;
        int testwidth = 8;

        Logic.CellType[][] field = new Logic.CellType[testheight][testwidth];
        for (int y = 0; y < testheight; y++) {
            for (int x = 0; x < testwidth; x++) {
                field[y][x] = Logic.CellType.FLOOR;
            }
        }
        final Map<Logic.Pos, Logic.ThingType> thingTypeMap = new HashMap<>();
        thingTypeMap.put(new Logic.Pos(0, 0), Logic.ThingType.PLAYER);

        Logic logic = new Logic(field, thingTypeMap);


        
    }

}
