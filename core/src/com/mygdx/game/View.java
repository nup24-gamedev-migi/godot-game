package com.mygdx.game;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.*;
import com.badlogic.gdx.graphics.VertexAttributes.Usage;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.graphics.g3d.Material;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelBatch;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.attributes.BlendingAttribute;
import com.badlogic.gdx.graphics.g3d.attributes.TextureAttribute;
import com.badlogic.gdx.graphics.g3d.decals.CameraGroupStrategy;
import com.badlogic.gdx.graphics.g3d.decals.Decal;
import com.badlogic.gdx.graphics.g3d.decals.DecalBatch;
import com.badlogic.gdx.graphics.g3d.utils.MeshPartBuilder;
import com.badlogic.gdx.graphics.g3d.utils.ModelBuilder;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.Vector2;
import com.badlogic.gdx.math.Vector3;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class View {
  private final ShapeRenderer debugRenderer;
  private final SpriteBatch batch;
  private final Texture playerImg;
  private final Texture exit;
  private final Texture boxImg;
  private final Texture[] grass;
  private final Texture[] side;
  private final Texture wall;
  private final Texture shadow;
  private final Texture shadowHand;
  private final Texture chest;

  /* Resources for the don't starve look */
  private final ModelBatch modelBatch;
  private List<Model> shadowModels;
  private final Camera cam;
  private final DecalBatch decalBatch;

  private static final float sizeOfBlock = 1 / 10f * 2;
  private static final float wallHeight = 0.8f;
  private final Texture[] traceTexture;
  private static final Logic.MoveDirection[] dirs = new Logic.MoveDirection[]{
          Logic.MoveDirection.LEFT,
          Logic.MoveDirection.RIGHT,
          Logic.MoveDirection.UP,
          Logic.MoveDirection.DOWN
  };

  View() {
    playerImg = new Texture("char.png");
    boxImg = new Texture("box.png");
//        grass = new Texture[] { new Texture("boxy.png") };
    grass = IntStream.range(1, 7)
            .mapToObj(x -> new Texture("grass" + x + ".png"))
            .toArray(Texture[]::new);
    side = IntStream.range(1, 4)
            .mapToObj(x -> new Texture("flooredge" + x + ".png"))
            .toArray(Texture[]::new);
    wall = new Texture("wall.png");
    debugRenderer = new ShapeRenderer();
    batch = new SpriteBatch();
    shadow = new Texture("shadow4.png");
    shadowHand = new Texture("shadowhand2.png");
    chest = new Texture("chest.png");
    exit = new Texture("trapdoor.png");
    traceTexture = IntStream.range(1, 4)
            .mapToObj(x -> new Texture("trace" + x + ".png"))
            .toArray(Texture[]::new);

    modelBatch = new ModelBatch();
    cam = new PerspectiveCamera(30, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());

    decalBatch = new DecalBatch(10, new CameraGroupStrategy(cam));
    shadowModels = Collections.emptyList();
  }

  public void view(final Logic model) {
    cam.position.set(cameraPos(model));
    cam.lookAt(fieldLookAtPoint(model));
    cam.far = 300f;
    cam.update();

    Gdx.gl.glViewport(0, 0, Gdx.graphics.getWidth(), Gdx.graphics.getHeight());
    Gdx.gl.glClear(GL20.GL_COLOR_BUFFER_BIT | GL20.GL_DEPTH_BUFFER_BIT);

    drawField(model);
    drawWalls(model);
    drawPlayerTrace(model);
    decalBatch.flush();

    modelBatch.begin(cam);
    // FIXME this is some lame shit right here
    if (shadowModels.isEmpty() && model.isTreasureStolen()) {
      shadowModels = buildShadowSegments(model)
              .stream()
              .map(x -> buildShadow(x, shadow, shadowHand))
              .collect(Collectors.toList());
    }
    if (!shadowModels.isEmpty() && model.isTreasureStolen()) {
      shadowModels.stream()
              .map(ModelInstance::new)
              .forEach(modelBatch::render);
    } else if (!shadowModels.isEmpty()) {
      shadowModels.forEach(Model::dispose);
      shadowModels.clear();
    }
    modelBatch.end();

    model.allThings().forEach(entry -> {
      final Logic.ThingType ty = entry.getValue();
      final Texture img;
      final float shiftAlongFloor;
      final float scale;
      final boolean billboard;

      switch (ty) {
        case PLAYER: {
          img = playerImg;
          shiftAlongFloor = 0f;
          scale = 2f;
          billboard = false;
          break;
        }
        case BOX: {
          img = boxImg;
          shiftAlongFloor = sizeOfBlock / 2;
          scale = 1f;
          billboard = true;
          break;
        }
        default:
          throw new IllegalStateException("Unknown thing type!");
      }

      drawThing(
              scale,
              billboard,
              entry.getKey(),
              new TextureRegion(img),
              shiftAlongFloor
      );
    });

    decalBatch.flush();
  }

  private void drawThing(
          final float scale,
          final boolean billboard,
          final Logic.Pos lPos,
          final TextureRegion tex,
          final float shiftAlongFloor
  ) {
    Vector3 pos = logicToDisplay(lPos)
            .add(sizeOfBlock / 2f, -1f + sizeOfBlock / 2f, sizeOfBlock / 2f)
            .add(0, 0, shiftAlongFloor);
    final Decal dec = Decal.newDecal(sizeOfBlock, sizeOfBlock, tex);
    dec.setBlending(GL20.GL_SRC_ALPHA, GL20.GL_ONE_MINUS_SRC_ALPHA);

    dec.setPosition(pos);
    dec.setScale(scale);
    dec.transformationOffset = new Vector2(0, -sizeOfBlock / 2);
    if (billboard) {
      dec.lookAt(cam.position, cam.up);
    }

    decalBatch.add(dec);
  }

  private static Vector3 shadowWiggle(final Random rand) {
    return new Vector3(
            0.10f * sizeOfBlock / 2 * (2 * rand.nextFloat() - 1),
            0,
            0.10f * sizeOfBlock / 2 * (2 * rand.nextFloat() - 1)
    );
  }

  final Integer shadowStart(final Logic logic) {
    //final Logic.Pair pair = logic.getHistory().get(0);
    //final Logic.Pos t = pair.pos.applyDir(pair.dir);

//        return new Logic.Pos(
//                pair.pos.x - (t.x - pair.pos.x),
//                pair.pos.y - (t.y - pair.pos.y)
//        );
    final List<Logic.Pair> history = logic.getHistory();
    for (int i = 0; i < history.size(); i++) {
      final Logic.Pos pos = history.get(i).pos;
      if (logic.getCell(pos.x, pos.y).hasShadow) {
        return i;
      }
    }
    return null;
  }

  private boolean isDir(final Logic.Pos l, final Logic.Pos r) {
    return l.equals(r.applyDir(Logic.MoveDirection.LEFT)) ||
            l.equals(r.applyDir(Logic.MoveDirection.RIGHT)) ||
            l.equals(r.applyDir(Logic.MoveDirection.DOWN)) ||
            l.equals(r.applyDir(Logic.MoveDirection.UP));
  }

  private List<List<Logic.Pos>> buildShadowSegments(final Logic logic) {
    final List<List<Logic.Pos>> res = new ArrayList<>();

    Integer startIndex = shadowStart(logic);
    if (startIndex == null) {
      return res;
    }

    final Logic.Pos[] prev = {logic.getHistory().get(startIndex).pos};

    res.add(new ArrayList<>(Collections.singleton(prev[0])));

    logic.getHistory().stream().skip(startIndex + 1).forEach(pair -> {
      final Logic.Pos end = pair.pos;
      if (!logic.getCell(end.x, end.y).hasShadow) {
        return;
      }

      if (!isDir(prev[0], end) && !res.get(res.size() - 1).isEmpty()) {
        prev[0] = end;

        res.add(new ArrayList<>());
        res.get(res.size() - 1).add(prev[0]);
        return;
      }

      prev[0] = end;
      res.get(res.size() - 1).add(end);
    });

    return res;
  }

  private static Model buildShadow(
          final List<Logic.Pos> points,
          final Texture shadow,
          final Texture shadowHand) {
    final ModelBuilder builder = new ModelBuilder();
    builder.begin();
    final Random rand = new Random();
    Vector3 prev = logicToDisplay(points.get(0))
            .add(sizeOfBlock / 2, -0.99f, sizeOfBlock / 2);

    // FIXME we probably might need a unique visual for 1-cell shadow
    if (points.size() == 1) {
      final MeshPartBuilder handBuilder = builder.part(
              "hand",
              GL20.GL_TRIANGLES,
              Usage.Position | Usage.TextureCoordinates,
              new Material(
                      new BlendingAttribute(true, 1),
                      TextureAttribute.createDiffuse(new TextureRegion(shadowHand))
              )
      );

      prev.add(-sizeOfBlock / 2, 0, 0);
      final Logic.Pos pos = points.get(points.size() - 1);
      final Vector3 end = logicToDisplay(pos)
              .add(sizeOfBlock / 2, -0.99f, sizeOfBlock / 2);
      final Vector3 dir = end.cpy().sub(prev).nor();
      final Vector3 perpNo = new Vector3(dir.z, 0, -dir.x);
      final Vector3 perp = new Vector3(dir.z, 0, -dir.x)
              .scl(sizeOfBlock / 2);
      final Vector3 ranoff = dir.cpy().scl(0.3f * sizeOfBlock / 2 * (Math.min(rand.nextFloat() + 0.2f, 1)))
              .add(perpNo.scl(sizeOfBlock / 2 * (
                      (0.3f * rand.nextFloat() + 0.2f) *
                              (rand.nextBoolean() ? -1 : 1)
              )));
      final Vector3 off = dir.cpy().scl(sizeOfBlock / 2 * 0.2f);
      final Vector3 visend = end.cpy().add(
              dir.cpy().scl(sizeOfBlock / 2 * 1.2f)
      );
      final Vector3 prevOff = dir.cpy().scl(-sizeOfBlock / 2 * 0.4f);

      handBuilder.rect(
              prev.cpy().add(prevOff).add(perp.cpy().scl(0.9f)),
              prev.cpy().add(prevOff).sub(perp.cpy().scl(0.9f)),
              visend.cpy().add(off).sub(perp.cpy().scl(1.2f)),
              visend.cpy().add(off).add(perp.cpy().scl(1.2f)),
              Vector3.Y
      );

      return builder.end();
    }

    final MeshPartBuilder bodyBuilder = builder.part(
            "arm",
            GL20.GL_TRIANGLES,
            Usage.Position | Usage.TextureCoordinates,
            new Material(
                    new BlendingAttribute(true, 1),
//                        ColorAttribute.createDiffuse(Color.BLACK)
                    TextureAttribute.createDiffuse(new TextureRegion(shadow))
            )
    );

    for (int i = 1; i < points.size(); i++) {
      final Logic.Pos pos = points.get(i);
      final Vector3 end = logicToDisplay(pos)
              .add(sizeOfBlock / 2, -0.99f, sizeOfBlock / 2);
      final Vector3 dir = end.cpy().sub(prev).nor();
      final Vector3 perpNo = new Vector3(dir.z, 0, -dir.x);
      final Vector3 perp = new Vector3(dir.z, 0, -dir.x)
              .scl(sizeOfBlock / 2);
      final Vector3 ranoff = dir.cpy().scl(0.3f * sizeOfBlock / 2 * (Math.min(rand.nextFloat() + 0.2f, 1)))
              .add(perpNo.scl(sizeOfBlock / 2 * (
                      (0.3f * rand.nextFloat() + 0.2f) *
                              (rand.nextBoolean() ? -1 : 1)
              )));
      final Vector3 off = dir.cpy().scl(sizeOfBlock / 2 * 0.2f);
      final Vector3 visend;
      final float endWidth;

      if (i == points.size() - 1) {
        endWidth = 0.2f;
        visend = end.cpy().add(dir.cpy().scl(-sizeOfBlock / 2 * 0.4f));
      } else {
        endWidth = 0.45f;
        visend = end.cpy().add(off).add(ranoff);
      }

      bodyBuilder.rect(
              prev.cpy().add(dir.cpy().scl(-sizeOfBlock / 2 * 0.1f)).add(perp.cpy().scl(0.35f)).add(shadowWiggle(rand)),
              prev.cpy().add(dir.cpy().scl(-sizeOfBlock / 2 * 0.1f)).sub(perp.cpy().scl(0.35f)).add(shadowWiggle(rand)),
              visend.cpy().sub(perp.cpy().scl(endWidth)).add(shadowWiggle(rand)),
              visend.cpy().add(perp.cpy().scl(endWidth)).add(shadowWiggle(rand)),
//                    prev.cpy().add(perp.cpy().scl(0.55f)).add(shadowWiggle(rand)),
//                    prev.cpy().sub(perp.cpy().scl(0.55f)).add(shadowWiggle(rand)),
//                    visend.cpy().add(off).sub(perp.cpy().scl(0.65f)).add(shadowWiggle(rand)),
//                    visend.cpy().add(off).add(perp.cpy().scl(0.65f)).add(shadowWiggle(rand)),
              Vector3.Y
      );

      prev = visend;
    }

    final MeshPartBuilder handBuilder = builder.part(
            "hand",
            GL20.GL_TRIANGLES,
            Usage.Position | Usage.TextureCoordinates,
            new Material(
                    new BlendingAttribute(true, 1),
                    TextureAttribute.createDiffuse(new TextureRegion(shadowHand))
            )
    );

    final Logic.Pos pos = points.get(points.size() - 1);
    final Vector3 end = logicToDisplay(pos)
            .add(sizeOfBlock / 2, -0.99f, sizeOfBlock / 2);
    final Vector3 dir = end.cpy().sub(prev).nor();
    final Vector3 perpNo = new Vector3(dir.z, 0, -dir.x);
    final Vector3 perp = new Vector3(dir.z, 0, -dir.x)
            .scl(sizeOfBlock / 2);
    final Vector3 ranoff = dir.cpy().scl(0.3f * sizeOfBlock / 2 * (Math.min(rand.nextFloat() + 0.2f, 1)))
            .add(perpNo.scl(sizeOfBlock / 2 * (
                    (0.3f * rand.nextFloat() + 0.2f) *
                            (rand.nextBoolean() ? -1 : 1)
            )));
    final Vector3 off = dir.cpy().scl(sizeOfBlock / 2 * 0.2f);
    final Vector3 visend = end.cpy().add(
            dir.cpy().scl(sizeOfBlock / 2 * 1.2f)
    );
    final Vector3 prevOff = dir.cpy().scl(-sizeOfBlock / 2 * 0.4f);

    handBuilder.rect(
            prev.cpy().add(prevOff).add(perp.cpy().scl(0.9f)),
            prev.cpy().add(prevOff).sub(perp.cpy().scl(0.9f)),
            visend.cpy().add(off).sub(perp.cpy().scl(1.2f)),
            visend.cpy().add(off).add(perp.cpy().scl(1.2f)),
            Vector3.Y
    );

    return builder.end();
  }

  private void drawPlayerTrace(final Logic logic) {
    if (logic.isTreasureStolen()) {
      return;
    }

    for (final Logic.Pair pair : logic.getHistory()) {
      // pair contains an old pos.
      final Logic.Pos t = pair.pos.applyDir(pair.dir);
      final Logic.Pos oldPos = new Logic.Pos(
              pair.pos.x - (t.x - pair.pos.x),
              pair.pos.y - (t.y - pair.pos.y)
      );

      final Vector3 beg = logicToDisplay(oldPos)
              .add(sizeOfBlock / 2, 0, sizeOfBlock / 2);
      final Vector3 end = logicToDisplay(pair.pos)
              .add(sizeOfBlock / 2, 0, sizeOfBlock / 2);
      final Vector3 tracePos = beg.cpy().scl(0.5f)
              .add(end.cpy().scl(0.5f));
      tracePos.y = -0.99f;

      //DrawDebugLine(beg, end);
      int x = pair.pos.x;
      int y = pair.pos.y;
      final Decal dec = Decal.newDecal(
              sizeOfBlock, sizeOfBlock * 0.45f,
              new TextureRegion(traceTexture[((x << 16) ^ y) % traceTexture.length])
      );
      dec.setColor(1, 1, 1, 0.6f);
      dec.setBlending(GL20.GL_SRC_ALPHA, GL20.GL_ONE_MINUS_SRC_ALPHA);
      dec.rotateX(-90);
      dec.setPosition(tracePos);

      switch (pair.dir) {
        case DOWN: {
          dec.rotateZ(-90);
          break;
        }
        case LEFT: {
          dec.rotateZ(180);
          break;
        }
        case UP: {
          dec.rotateZ(90);
          break;
        }
      }

      decalBatch.add(dec);
    }
  }

  private void drawWalls(final Logic logic) {
    final int width = logic.getFieldWidth();
    final int height = logic.getFieldHeight();
    final Vector3 center = fieldCenter(logic);

    final Decal leftWall = Decal.newDecal(
            (float) height * sizeOfBlock,
            wallHeight,
            new TextureRegion(wall) // TODO improve
    );
    leftWall.rotateY(90);
    leftWall.setPosition(
            logicToDisplay(new Logic.Pos(0, 0)).x,
            wallHeight / 2f - 1f,
            center.z
    );

    final Decal rightWall = Decal.newDecal(
            (float) height * sizeOfBlock,
            wallHeight,
            new TextureRegion(wall) // TODO improve
    );
    rightWall.rotateY(-90);
    rightWall.setPosition(
            logicToDisplay(new Logic.Pos(width, 0)).x,
            wallHeight / 2f - 1f,
            center.z
    );

    final Decal backWall = Decal.newDecal(
            (float) width * sizeOfBlock,
            wallHeight,
            new TextureRegion(wall) // TODO improve
    );
    backWall.setPosition(
            center.x,
            wallHeight / 2f - 1f,
            logicToDisplay(new Logic.Pos(0, 0)).z
    );

    decalBatch.add(leftWall);
    decalBatch.add(rightWall);
    decalBatch.add(backWall);
  }

  private void drawTileEdge(
          final int ridx,
          final Logic.MoveDirection dir,
          final Vector3 basePos
  ) {
    final int off;
    switch (dir) {
      case UP: {
        off = 0;
        break;
      }
      case RIGHT: {
        off = 1;
        break;
      }
      case DOWN: {
        off = 2;
        break;
      }
      case LEFT: {
        off = 3;
        break;
      }
      default: {
        throw new IllegalStateException("Unknown dir" + dir);
      }
    }
    final Decal dec = Decal.newDecal(
            sizeOfBlock, sizeOfBlock / 3,
            new TextureRegion(side[(ridx + off) % side.length])
    );
    final Vector3 posOff;
    switch (dir) {
      case LEFT: {
        posOff = new Vector3(-1, -1 / 3f, 0);
        break;
      }
      case RIGHT: {
        posOff = new Vector3(1, -1 / 3f, 0);
        break;
      }
      case UP: {
        posOff = new Vector3(0, -1 / 3f, -1);
        break;
      }
      case DOWN: {
        posOff = new Vector3(0, -1 / 3f, 1);
        break;
      }
      default: {
        throw new IllegalStateException("Unknown dir" + dir);
      }
    }

    dec.setColor(0.8f, 0.8f, 0.8f, 1.0f);
    dec.rotateY((float) (90 * off));
    dec.setPosition(basePos.cpy().add(posOff.scl(sizeOfBlock / 2)));

    decalBatch.add(dec);
  }

  private void drawTileEdge(
          final int ridx,
          final Logic.MoveDirection dir,
          final Logic logic,
          final Logic.Pos pos,
          final Vector3 basePos
  ) {
    final Logic.Pos checkPos = pos.applyDir(dir);
    final Logic.Cell cell = logic.getCell(checkPos.x, checkPos.y);

    if (cell == null) {
      return;
    }

    if (
            cell.type != Logic.CellType.FLOOR &&
                    cell.type != Logic.CellType.ENTRANCE &&
                    cell.type != Logic.CellType.TREASURE
    ) {
      return;
    }

    drawTileEdge(ridx, dir, basePos);
  }

  private void drawField(final Logic logic) {
    for (int y = 0; y < logic.getFieldHeight(); y++) {
      for (int x = 0; x < logic.getFieldWidth(); x++) {
        Vector3 currentCellPos = logicToDisplay(
                new Logic.Pos(x, y)
        ).add(sizeOfBlock / 2, -1, sizeOfBlock / 2);
        final Logic.Cell cell = logic.getCell(x, y);
        final int ridx = (x << 16) ^ y;
        final Logic.Pos cellLogPos = new Logic.Pos(x, y);

        final Texture tileTexture;
        switch (cell.type) {
          case FLOOR:
          case TREASURE: {
            tileTexture = grass[((x << 16) ^ y) % grass.length];
            break;
          }
          case WALL: {
            tileTexture = null;
            break;
          }
          case ENTRANCE: {
            tileTexture = exit;
            break;
          }
          default: {
            throw new IllegalStateException("Unknown tile type!");
          }
        }

        if (tileTexture == null) {
          for (final Logic.MoveDirection dir : dirs) {
            drawTileEdge(ridx, dir, logic, cellLogPos, currentCellPos);
          }
          continue;
        }

        final Decal dec = Decal.newDecal(
                sizeOfBlock, sizeOfBlock,
                new TextureRegion(tileTexture)
        );
        dec.rotateX(-90);
        dec.setPosition(currentCellPos);

        decalBatch.add(dec);

        /* exception for the chest */
        if (!logic.isTreasureStolen() && cell.type == Logic.CellType.TREASURE) {
          drawThing(
                  1.3f,
                  true,
                  new Logic.Pos(x, y),
                  new TextureRegion(chest),
                  sizeOfBlock / 2
          );
        }
      }
    }
  }

  private Vector3 cameraPos(final Logic logic) {
    final Vector3 camPos = fieldLookAtPoint(logic);

    camPos.y = 1.5f + (1 - (float) logic.getFieldHeight() / 10f) * 1.2f;
    camPos.z = 3.0f - (1 - (float) logic.getFieldHeight() / 10f) * 3.0f;

    return camPos;
  }

  private Vector3 fieldLookAtPoint(final Logic logic) {
    final Vector3 center = fieldCenter(logic);

    /* Looking at a point slightly above -1f improves the overall feel */
    center.y = -0.8f;

    return center;
  }

  private Vector3 fieldCenter(final Logic logic) {
    final int width = logic.getFieldWidth();
    final int height = logic.getFieldHeight();
    final Vector3 center = logicToDisplay(new Logic.Pos(width / 2, height / 2));

    /* For evenly sized maps it's not the center of any tile */
    if (width % 2 == 1) {
      center.x += sizeOfBlock / 2f;
    }

    if (height % 2 == 1) {
      center.z += sizeOfBlock / 2f;
    }

    /* The actual y of tiles */
    center.y = -1f;

    return center;
  }

  public static Vector3 logicToDisplay(
          final Logic.Pos lPos
  ) {
    return new Vector3(
            (float) lPos.x,
            0f,
            (float) lPos.y
    ).scl(sizeOfBlock).add(-1, 0, -1);
  }

  public void dispose() {
    debugRenderer.dispose();
    batch.dispose();

    modelBatch.dispose();
    shadowModels.forEach(Model::dispose);
  }
}
