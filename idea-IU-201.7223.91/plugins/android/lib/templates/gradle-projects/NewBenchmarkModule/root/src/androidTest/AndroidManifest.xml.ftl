<?xml version="1.0" encoding="utf-8"?>
<manifest
    xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:tools="http://schemas.android.com/tools"
    package="${packageName}">

    <!--
      Important: disable debugging for accurate performance results

      In a com.android.library project, this flag must be disabled from this
      manifest, as it is not possible to override this flag from Gradle.
    -->
    <application
        android:debuggable="false"
        tools:ignore="HardcodedDebugMode"
        tools:replace="android:debuggable"/>
</manifest>