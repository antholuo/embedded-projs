/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.h
  * @brief          : Header for main.c file.
  *                   This file contains the common defines of the application.
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2024 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_hal.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Exported types ------------------------------------------------------------*/
/* USER CODE BEGIN ET */

/* USER CODE END ET */

/* Exported constants --------------------------------------------------------*/
/* USER CODE BEGIN EC */

/* USER CODE END EC */

/* Exported macro ------------------------------------------------------------*/
/* USER CODE BEGIN EM */

/* USER CODE END EM */

/* Exported functions prototypes ---------------------------------------------*/
void Error_Handler(void);

/* USER CODE BEGIN EFP */

/* USER CODE END EFP */

/* Private defines -----------------------------------------------------------*/
#define GPIO_INPUT_FROM_FC_Pin GPIO_PIN_13
#define GPIO_INPUT_FROM_FC_GPIO_Port GPIOC
#define GPIO_OUT_TO_FC_Pin GPIO_PIN_14
#define GPIO_OUT_TO_FC_GPIO_Port GPIOC
#define GPIO_OUT_TO_RPI_Pin GPIO_PIN_15
#define GPIO_OUT_TO_RPI_GPIO_Port GPIOC
#define GPIO_OUT_GRFC1_Pin GPIO_PIN_0
#define GPIO_OUT_GRFC1_GPIO_Port GPIOC
#define GPIO_OUT_GRFC2_Pin GPIO_PIN_1
#define GPIO_OUT_GRFC2_GPIO_Port GPIOC
#define ADC1_IN13_AMux_IN_Pin GPIO_PIN_3
#define ADC1_IN13_AMux_IN_GPIO_Port GPIOC
#define GPIO_OUT_WakeupIn_Pin GPIO_PIN_4
#define GPIO_OUT_WakeupIn_GPIO_Port GPIOC
#define GPIO_OUT_W_Disable_Pin GPIO_PIN_5
#define GPIO_OUT_W_Disable_GPIO_Port GPIOC
#define GPIO_OUT_AP_Ready_Pin GPIO_PIN_0
#define GPIO_OUT_AP_Ready_GPIO_Port GPIOB
#define TIM3_CH4_GPS_PPS_Pin GPIO_PIN_1
#define TIM3_CH4_GPS_PPS_GPIO_Port GPIOB
#define TIM8_CH3_NEOPIXEL_Pin GPIO_PIN_8
#define TIM8_CH3_NEOPIXEL_GPIO_Port GPIOC
#define TIM8_CH4_IR_Pin GPIO_PIN_9
#define TIM8_CH4_IR_GPIO_Port GPIOC
#define TIM1_CH1_HB_LED_1_Pin GPIO_PIN_8
#define TIM1_CH1_HB_LED_1_GPIO_Port GPIOA
#define TIM1_CH2_HB_LED_2_Pin GPIO_PIN_9
#define TIM1_CH2_HB_LED_2_GPIO_Port GPIOA
#define TIM1_CH3_HB_LED_3_Pin GPIO_PIN_10
#define TIM1_CH3_HB_LED_3_GPIO_Port GPIOA
#define GPIO_Out_AnalogMux_Pin GPIO_PIN_15
#define GPIO_Out_AnalogMux_GPIO_Port GPIOA
#define GPIO_Out_AnalogMuxB4_Pin GPIO_PIN_4
#define GPIO_Out_AnalogMuxB4_GPIO_Port GPIOB
#define GPIO_Out_AnalogMuxB5_Pin GPIO_PIN_5
#define GPIO_Out_AnalogMuxB5_GPIO_Port GPIOB
#define GPIO_IN_PWR_GOOD2_Pin GPIO_PIN_8
#define GPIO_IN_PWR_GOOD2_GPIO_Port GPIOB
#define GPIO_IN_PWR_GOOD1_Pin GPIO_PIN_9
#define GPIO_IN_PWR_GOOD1_GPIO_Port GPIOB

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
