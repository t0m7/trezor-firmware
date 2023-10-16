#ifndef MODELS_MODEL_T1B1_H_
#define MODELS_MODEL_T1B1_H_

#define MODEL_NAME "1"
#define MODEL_FULL_NAME "Trezor Model One"
#define MODEL_INTERNAL_NAME "T1B1"
#define MODEL_INTERNAL_NAME_TOKEN T1B1
#define MODEL_INTERNAL_NAME_QSTR MP_QSTR_T1B1

#define BOOTLOADER_START 0x08000000
#define FIRMWARE_START 0x08010000

#define IMAGE_CHUNK_SIZE (64 * 1024)
#define BOOTLOADER_IMAGE_MAXSIZE (32 * 1024 * 1)  // 32 KB
#define FIRMWARE_IMAGE_MAXSIZE (64 * 1024 * 15)   // 960 KB
#define NORCOW_SECTOR_SIZE (16 * 1024)

#endif
