package com.mygdx.game;

import com.badlogic.gdx.*;

import java.util.HashMap;
import java.util.LinkedHashMap;

public class MyGame extends ApplicationAdapter {
    private View view;
    private Logic logic;


    private final String[][][] level = new String[][][] {
            {
                    {"x", "x", "x", "x", "x", "x", "x"},
                    {"x", "o", "o", "o", "o", "t", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "e", "o", "o", "o", "o", "x"},
                    {"x", "x", "x", "x", "x", "x", "x"}
            },
            {
                    {"x", "x", "x", "x", "x", "x", "x"},
                    {"e", "o", "o", "o", "o", "o", "t"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
                    {"x", "o", "o", "o", "o", "o", "x"},
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
        logic = loadHardcodedLevelAndGenerateLogic(1);
        view = new View();

        InputProcessor inputProcessor = new InputProcessor() {
            @Override
            public boolean keyDown(int keycode) {
                switch (keycode) {
                    case Input.Keys.A: {
                        logic.movePlayer(Logic.MoveDirection.LEFT);
                        break;
                    }
                    case Input.Keys.D: {
                        logic.movePlayer(Logic.MoveDirection.RIGHT);
                        break;
                    }
                    case Input.Keys.W: {
                        logic.movePlayer(Logic.MoveDirection.UP);
                        break;
                    }
                    case Input.Keys.S: {
                        logic.movePlayer(Logic.MoveDirection.DOWN);
                        break;
                    }
                    case Input.Keys.K: {
                        logic.applyShadowToField();
                        break;
                    }
                    case Input.Keys.NUM_1: {
                        logic = loadHardcodedLevelAndGenerateLogic(1);
                        break;
                    }
                    case Input.Keys.NUM_2: {
                        logic = loadHardcodedLevelAndGenerateLogic(2);
                        break;
                    }
                    case Input.Keys.NUM_3: {
                        logic = loadHardcodedLevelAndGenerateLogic(3);
                        break;
                    }
                    case Input.Keys.NUM_4: {
                        logic = loadHardcodedLevelAndGenerateLogic(4);
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
    Logic loadHardcodedLevelAndGenerateLogic(int n) {
        final String[][] levelTemplate = level[n - 1];
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


    @Override
    public void render() {
        view.view(logic);
    }

    @Override
    public void dispose() {
        view.dispose();
    }
}
