
import QtQuick 2.12
import QtQuick.Window 2.12
import calc.ui.backend 1.0

Window {
    visible: true
    width: 640
    height: 480
    title: qsTr("Hello World")

    Backend {
        id: backend
    }

    CalculatorForm {
        anchors.fill: parent
        width: width
        height: height
    }
}
