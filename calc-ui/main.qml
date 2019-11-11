
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

        onAnswered: screen_text.text = str
    }

    Item {
        id: parentitem
        anchors.fill: parent
        width: width
        height: height

        Column {
            id: column
            spacing: 0
            anchors.fill: parent

            Rectangle {
                id: screen
                height: 100
                color: "#ffffff"
                anchors.right: parent.right
                anchors.rightMargin: 0
                anchors.left: parent.left
                anchors.leftMargin: 0

                TextInput {
                    id: screen_text
                    inputMask: ""
                    anchors.rightMargin: 15
                    anchors.leftMargin: 15
                    transformOrigin: Item.Center
                    anchors.fill: parent
                    anchors.topMargin: 21
                    cursorVisible: true
                    horizontalAlignment: Text.AlignRight
                    font.pixelSize: 40
                }
            }

            Grid {
                id: grid
                height: parent.height - screen.height
                anchors.right: parent.right
                anchors.rightMargin: 0
                anchors.left: parent.left
                anchors.leftMargin: 0
                spacing: 0
                rows: 4
                columns: 5

                CalcButton {
                    id: button_7
                    text: qsTr("7")
                    grid: parent
                }

                CalcButton {
                    id: button_8
                    text: qsTr("8")
                    grid: parent
                }

                CalcButton {
                    id: button_9
                    text: qsTr("9")
                    grid: parent
                }

                CalcButton {
                    id: button_div
                    text: qsTr("÷")
                    grid: parent
                }

                CalcButton {
                    id: button_clear
                    text: qsTr("C")
                    grid: parent
                    writeToScreen: false
                }

                CalcButton {
                    id: button_4
                    text: qsTr("4")
                    grid: parent
                }

                CalcButton {
                    id: button_5
                    text: qsTr("5")
                    grid: parent
                }

                CalcButton {
                    id: button_6
                    text: qsTr("6")
                    grid: parent
                }

                CalcButton {
                    id: button_mult
                    text: qsTr("×")
                    grid: parent
                }

                CalcButton {
                    id: button_empty1
                    text: qsTr("")
                    enabled: false
                    checkable: false
                    grid: parent
                }

                CalcButton {
                    id: button_1
                    text: qsTr("1")
                    grid: parent
                }

                CalcButton {
                    id: button_2
                    text: qsTr("2")
                    grid: parent
                }

                CalcButton {
                    id: button_3
                    text: qsTr("3")
                    grid: parent
                }

                CalcButton {
                    id: button_minus
                    text: qsTr("−")
                    grid: parent
                }

                CalcButton {
                    id: button_empty2
                    text: qsTr("")
                    enabled: false
                    grid: parent
                }

                CalcButton {
                    id: button_0
                    text: qsTr("0")
                    grid: parent
                }

                CalcButton {
                    id: button_dot
                    text: qsTr(".")
                    grid: parent
                }

                CalcButton {
                    id: button_empty3
                    text: qsTr("")
                    enabled: false
                    grid: parent
                }

                CalcButton {
                    id: button_plus
                    text: qsTr("+")
                    grid: parent
                }

                CalcButton {
                    id: button_equals
                    text: qsTr("=")
                    grid: parent
                    writeToScreen: false
                    onClicked: backend.solve()
                }
            }
        }

        Connections {
            target: screen_text
            onTextChanged: backend.updatedTextField(screen_text.text)
        }
    }
}
