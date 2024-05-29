package com.mygdx.game;

import com.badlogic.gdx.*;

import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;

public class MyGame extends ApplicationAdapter {
    private View view;
    private Logic logic;
    private int levelNumber;


    private final String[][][] levels = new String[][][] {
            {
                    {"x", "x", "x", "x", "x"},
                    {"x", "o", "o", "t", "x"},
                    {"x", "o", "o", "o", "x"},
                    {"x", "e", "o", "o", "x"},
                    {"x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x"},
                    {"x", "o", "o", "o", "x"},
                    {"x", "o", "t", "o", "x"},
                    {"e", "o", "o", "o", "x"},
                    {"x", "x", "x", "x", "x"},

            },
            {
                    {"x", "x", "x", "x", "x"},
                    {"x", "o", "o", "o", "x"},
                    {"e", "o", "x", "o", "t"},
                    {"x", "o", "o", "o", "x"},
                    {"x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x"},
                    {"x", "o", "o", "o", "x"},
                    {"e", "o", "x", "o", "t"},
                    {"x", "o", "b", "x", "x"},
                    {"x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x", "x", "x"},
                    {"x", "o", "o", "o", "o", "o", "o"},
                    {"e", "o", "b", "o", "o", "x", "t"},
                    {"x", "o", "o", "o", "o", "x", "o"},
                    {"x", "x", "x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x", "x", "x"},
                    {"x", "o", "b", "x", "o", "o", "x"},
                    {"e", "o", "o", "o", "o", "o", "t"},
                    {"x", "o", "o", "x", "b", "o", "x"},
                    {"x", "x", "x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x", "x", "x"},
                    {"x", "o", "o", "b", "x", "o", "x"},
                    {"e", "o", "b", "b", "x", "o", "t"},
                    {"x", "o", "o", "o", "x", "o", "x"},
                    {"x", "x", "x", "x", "x", "x", "x"}
            },
            {
                    {"o", "o", "x", "x", "x", "x", "x", "o"},
                    {"x", "x", "x", "o", "o", "o", "x", "o"},
                    {"x", "e", "o", "b", "o", "o", "x", "o"},
                    {"x", "x", "x", "o", "b", "o", "x", "o"},
                    {"x", "t", "x", "x", "b", "o", "x", "o"},
                    {"x", "o", "o", "o", "o", "o", "x", "x"},
                    {"x", "o", "o", "b", "b", "b", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "o", "x"},
                    {"x", "x", "x", "x", "x", "x", "x", "x"},
            },
            {
                    {"e", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "b", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "b", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "t", "o", "o", "o"},
                    {"o", "o", "o", "o", "o", "o", "o", "o", "o", "o"},
            },
    };

    @Override
    public void create() {
//        logic = new Logic(loadField(), thingTypeMap);
        levelNumber = 1;
        logic = loadHardcodedLevelAndGenerateLogic();
        view = new View();

        InputProcessor inputProcessor = new InputProcessor() {
            @Override
            public boolean keyDown(int keycode) {
                switch (keycode) {
                    case Input.Keys.A, Input.Keys.LEFT: {
                        logic.movePlayer(Logic.MoveDirection.LEFT);
                        break;
                    }
                    case Input.Keys.UP, Input.Keys.W: {
                        logic.movePlayer(Logic.MoveDirection.UP);
                        break;
                    }
                    case Input.Keys.D, Input.Keys.RIGHT: {
                        logic.movePlayer(Logic.MoveDirection.RIGHT);
                        break;
                    }
                    case Input.Keys.S, Input.Keys.DOWN: {
                        logic.movePlayer(Logic.MoveDirection.DOWN);
                        break;
                    }
                    case Input.Keys.R: {
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
//                    case Input.Keys.K: {
//                        logic.applyShadowToField();
//                        break;
//                    }
                    case Input.Keys.NUM_1: {
                        levelNumber = 1;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_2: {
                        levelNumber = 2;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_3: {
                        levelNumber = 3;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_4: {
                        levelNumber = 4;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_5: {
                        levelNumber = 5;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_6: {
                        levelNumber = 6;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                    case Input.Keys.NUM_7: {
                        levelNumber = 7;
                        logic = loadHardcodedLevelAndGenerateLogic();
                        break;
                    }
                }
                return true;
            }

            @Override
            public boolean keyUp(int keycode) {
                return false;
            }

            @Override
            public boolean keyTyped(char character) {
                return false;
            }

            @Override
            public boolean touchDown(int screenX, int screenY, int pointer, int button) {
                return false;
            }

            @Override
            public boolean touchUp(int screenX, int screenY, int pointer, int button) {
                return false;
            }

            @Override
            public boolean touchCancelled(int screenX, int screenY, int pointer, int button) {
                return false;
            }

            @Override
            public boolean touchDragged(int screenX, int screenY, int pointer) {
                return false;
            }

            @Override
            public boolean mouseMoved(int screenX, int screenY) {
                return false;
            }

            @Override
            public boolean scrolled(float amountX, float amountY) {
                return false;
            }
        };
        Gdx.input.setInputProcessor(inputProcessor);
    }
    /*
    * This is template fot loading class from libGDX interface
    * Have to implement new function loadLevelFromFile, that construct libGDX TileMap interface.
    * And then give it to this function.
    * TODO implement this method!
    */
    public Logic.CellType[][] loadField() {
        return null;
    }

    // for now this function is god killer
    Logic loadHardcodedLevelAndGenerateLogic() {
        final String[][] levelTemplate = levels[levelNumber - 1];
        Logic.CellType[][] field;
        HashMap<Logic.Pos, Logic.ThingType> objectsOnField = new LinkedHashMap<>();

        field = new Logic.CellType[levelTemplate.length][];

        for (int i = 0; i < levelTemplate.length; i++) {
            field[i] = new Logic.CellType[levelTemplate[i].length];
            for (int j = 0; j < levelTemplate[i].length; j++) {
                switch (levelTemplate[i][j]) {
                    case "o": {
                        field[i][j] = Logic.CellType.FLOOR;
                        break;
                    }
                    case "x": {
                        field[i][j] = Logic.CellType.WALL;
                        break;
                    }
                    case "t": {
                        field[i][j] = Logic.CellType.TREASURE;
                        break;
                    }
                    case "e": {
                        field[i][j] = Logic.CellType.ENTRANCE;
                        objectsOnField.put(new Logic.Pos(j, i), Logic.ThingType.PLAYER);
                        break;
                    }
                    case "b": {
                        field[i][j] = Logic.CellType.FLOOR;
                        objectsOnField.put(new Logic.Pos(j, i), Logic.ThingType.BOX);
                        break;
                    }
                }
            }
        }
        return new Logic(field, objectsOnField);
    }

    public void switchToNextLevel() {
        int maxLevelNumber = levels.length;
        if (levelNumber >= maxLevelNumber) {
            levelNumber = maxLevelNumber;
        } else {
            levelNumber += 1;
        }

        logic = loadHardcodedLevelAndGenerateLogic();
    }


    @Override
    public void render() {
        logic.tick();
        view.view(logic);
        if (logic.isTreasureStolen() && logic.isPlayerAtEntrace()) {
            switchToNextLevel();
        }
    }

    @Override
    public void dispose() {
        view.dispose();
    }
}
