#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include "backend.hpp"
#include "main.hpp"

extern "C" int runGui(Callbacks callbacks) {
    int argc = 0;

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

    QGuiApplication app(argc, nullptr);

    QQmlApplicationEngine engine;

    const QUrl url(QStringLiteral("qrc:/main.qml"));
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    engine.load(url);

    Backend backend(&app, callbacks);

    engine.rootContext()->setContextProperty("backend", &backend);

    return app.exec();
}
