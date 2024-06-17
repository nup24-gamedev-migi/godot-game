package com.mygdx.game;

import com.badlogic.gdx.backends.lwjgl3.Lwjgl3Application;
import com.badlogic.gdx.backends.lwjgl3.Lwjgl3ApplicationConfiguration;

// Please note that on macOS your application needs to be started with the -XstartOnFirstThread JVM argument
public class DesktopLauncher {
	public static void main (String[] arg) {
		Lwjgl3ApplicationConfiguration config = new Lwjgl3ApplicationConfiguration();
		config.setForegroundFPS(60);
		config.setTitle("test_gamedev_proj");
		config.setResizable(false);
		// FIXME these numbers were pulled out randomly just to not have a small window
		config.setWindowedMode(1800, 960);

		new Lwjgl3Application(new MyGame(), config);
	} 
}
