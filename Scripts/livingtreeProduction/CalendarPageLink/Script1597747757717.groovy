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

WebUI.click(findTestObject('LivingTree/CalendarLink/CalendarIndiaStandardTime'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/CalendarIndiaStandardTimeBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/CalendarCloseX'))

WebUI.click(findTestObject('LivingTree/CalendarLink/SubscribeButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/SubscribeBoxLT'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/SubscribeCloseX'))

WebUI.waitForElementVisible(findTestObject('LivingTree/CalendarLink/LeftSideDate'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/TodayButton'))

WebUI.click(findTestObject('LivingTree/CalendarLink/DayButton'))

WebUI.click(findTestObject('LivingTree/CalendarLink/MonthButton'))

WebUI.click(findTestObject('LivingTree/CalendarLink/EventButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/AddEventCalendarBOX'), 1)

WebUI.click(findTestObject('LivingTree/CalendarLink/CancelButton'))

WebUI.scrollToElement(findTestObject('LivingTreeProduction/CalendarLink/ScrollToTopButton'), 1)

WebUI.waitForElementVisible(findTestObject('LivingTree/CalendarLink/CalenderLeftSide'), 1)

WebUI.click(findTestObject('LivingTreeProduction/MyNetworkPage/ScienceGroup'))

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/span_Add Member'))

