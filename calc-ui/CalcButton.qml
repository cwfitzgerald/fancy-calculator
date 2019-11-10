import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Extras 1.4

Button {
    property Grid grid

    width: grid.width / grid.columns
    height: grid.height / grid.rows
    font.pixelSize: Math.min(height, width) / 2
}