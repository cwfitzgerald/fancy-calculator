#pragma once

#include <QObject>
#include "main.hpp"

class Backend : public QObject {
    Q_OBJECT
public:
    explicit Backend(QObject *parent = nullptr);

    void setCallbacks(Callbacks);

    ~Backend();

signals:
    void answered(QString str);

public slots:
    void updatedTextField(QString str);

private:
    Callbacks _callbacks;

    static Callbacks emptyCallbacks();
};
