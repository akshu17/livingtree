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

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.ATTENDANCE_EMAIL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/AttendanceAlert/SendAttendanceAlertsbox'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/AttendanceAlert/AttendanceAlertIcon'), 0)

WebUI.click(findTestObject('LivingTree/AttendanceAlert/SendAttendanceAlertsbox'))

WebUI.waitForElementPresent(findTestObject('LivingTree/AttendanceAlert/AttendanceAlertBox'), 0)

WebUI.mouseOver(findTestObject('LivingTree/AttendanceAlert/SchoolDrpoDown'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('LivingTree/AttendanceAlert/SchoolDrpoDown'))

WebUI.selectOptionByValue(findTestObject('LivingTree/AttendanceAlert/HinduSchool'), 'Hinduschool', false)

WebUI.acceptAlert()

WebUI.mouseOver(findTestObject('LivingTree/AttendanceAlert/HinduSchool'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('LivingTree/AttendanceAlert/HinduSchool'))

WebUI.delay(1)

WebUI.mouseOver(findTestObject('LivingTree/AttendanceAlert/SchoolDrpoDown'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/AttendanceAlert/ChooseFileButton'))

