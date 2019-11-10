import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Extras 1.4

Button {
    property Grid grid
    property bool writeToScreen: true

    width: grid.width / grid.columns
    height: grid.height / grid.rows
    font.pixelSize: Math.min(height, width) / 2

    onPressed: {
        if (writeToScreen) {
            screen_text.text += text
        }
    }
}
