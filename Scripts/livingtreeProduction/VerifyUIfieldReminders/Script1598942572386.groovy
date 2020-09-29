import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_TEACHER)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.click(findTestObject('LivingTree/ConversationPageLink/CalendarIcon'))

WebUI.click(findTestObject('LivingTree/CalendarLink/EventButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ReminderPage/ReminderField'), 1)

WebUI.check(findTestObject('LivingTree/ReminderPage/ReminderCheckBox'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ReminderPage/AdvanceField'), 1)

WebUI.uncheck(findTestObject('LivingTree/ReminderPage/AdvanceCheckBox'))

WebUI.check(findTestObject('LivingTree/ReminderPage/AdvanceCheckBox'))

WebUI.click(findTestObject('LivingTree/ReminderPage/DayField'))

WebUI.click(findTestObject('LivingTree/ReminderPage/DayField'))

WebUI.uncheck(findTestObject('LivingTree/ReminderPage/SameDayCheckBox'))

WebUI.check(findTestObject('LivingTree/ReminderPage/SameDayCheckBox'))

WebUI.click(findTestObject('LivingTree/ReminderPage/HoursBox'))

WebUI.click(findTestObject('LivingTree/ReminderPage/HoursBox'))

WebUI.closeBrowser()

