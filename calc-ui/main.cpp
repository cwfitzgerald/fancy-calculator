#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QVariant>
#include <QQuickItem>
#include <iostream>
#include "backend.hpp"
#include "main.hpp"

extern "C" int runGui(Callbacks callbacks) {
    int argc = 0;

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

    QGuiApplication app(argc, nullptr);

    qmlRegisterType<Backend>("calc.ui.backend", 1, 0, "Backend");

    QQmlApplicationEngine engine;

    const QUrl url(QStringLiteral("qrc:/main.qml"));
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    engine.load(url);

    auto backend_ptr = engine.rootObjects().at(0)->children().at(0)->children().at(0);

    auto backend = qobject_cast<Backend*>(backend_ptr);

    backend->setCallbacks(callbacks);

    return app.exec();
}

void setAnswer(Backend *internal, char *value) {
    emit internal->answered(QString(value));
}

void invalidInput(Backend *internal, char *fixed)
{
    emit internal->invalidInput(QString(fixed));
}
