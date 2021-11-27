
[内核编程](https://www.bilibili.com/video/BV1h4411C7Y6?p=3&t=4)

## 驱动编程

驱动管理器创建驱动对象  调用驱动方法：

~~~cpp
#include "ntddk.h"

// 驱动入口名称必须一致
NTSTATUS DriverEntry(PDRIVER_OBJECT pDriverObject, PUNICODE_STRING pRegistryPath) {
    KdPrint("driver loaded!") ;

    // 驱动卸载函数的设置 不要求函数名称特定 只要签名符合即可！
    pDriverObject->DriverUnload = MyDriverUnload ;

    return STATUS_SUCCESS ;
}

VOID MyDriverUnload(PDRIVER_OBJECT pDriverObject){
    KdPrint("driver unloaded!") ;
}

NTSTATUS CreateDivece(PDRIVER_OBJECT pDriverObject) {

    status = IoCreateDivece(...) ;

    if(! NT_SUCCESS(status)){
        return status ;
    }
    // 设备对象属性设置 ...
    return status ;
}

~~~