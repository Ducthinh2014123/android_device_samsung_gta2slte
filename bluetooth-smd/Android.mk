LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE := libbt-vendor
LOCAL_MODULE_OWNER := samsung
LOCAL_VENDOR_MODULE := true
LOCAL_MULTILIB := 64
LOCAL_MODULE_TAGS := optional

QCOM_BT_SMD_PATH := hardware/qcom/bt/msm8909/libbt-vendor

LOCAL_SRC_FILES := \
    ../../../../$(QCOM_BT_SMD_PATH)/src/bt_vendor_qcom.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/hardware.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/hci_uart.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/hci_smd.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/hw_rome.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/hw_ar3k.c \
    ../../../../$(QCOM_BT_SMD_PATH)/src/bt_vendor_persist.cpp

LOCAL_C_INCLUDES := \
    $(QCOM_BT_SMD_PATH)/include \
    system/bt/hci/include \
    $(TARGET_OUT_HEADERS)/bt/hci_qcomm_init

LOCAL_SHARED_LIBRARIES := \
    libcutils \
    liblog \
    libbtnv

LOCAL_CFLAGS += \
     \
    -Wno-error

LOCAL_CXX_STL := none

include $(BUILD_SHARED_LIBRARY)
