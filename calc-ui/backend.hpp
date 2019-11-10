#pragma once

#include <QObject>
#include "main.hpp"

class Backend : public QObject {
    Q_OBJECT
public:
    explicit Backend(QObject *parent, Callbacks callbacks);

    ~Backend();

signals:

public slots:
    void updatedTextField(QString str);

private:
    Callbacks _callbacks;
};
