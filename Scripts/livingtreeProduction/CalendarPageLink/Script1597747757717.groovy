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

WebUI.waitForElementVisible(findTestObject('LivingTree/CalendarLink/CalenderLeftSide'), 1)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/ScrollToTopButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.delay(1)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/span_Add Member'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/Page_LivingTree David Samson/InviteParent(s)usinggroupcode'))

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/CalendarLink/ParentsGroupButtonClose'))

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/span_Add Member'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/Page_LivingTree David Samson/a_Invite Parent(s) via e-mail'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentInviteBox'), 0)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParentviaButtonClose'))

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/span_Add Member'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/Page_LivingTree David Samson/InviteParent(s)usinggroupcode'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ManagegroupcodeRymsParents'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentGroupCodePage'), 0)

WebUI.delay(0)

WebUI.click(findTestObject('LivingTree/CalendarLink/ParentsGroupButtonClose'))

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/span_Add Member'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/RymsGroupBox'), 0)

WebUI.click(findTestObject('LivingTreeProduction/CalendarLink/Page_LivingTree David Samson/a_Invite Student(s) via e-mail'))

WebUI.waitForElementPresent(findTestObject('LivingTree/CalendarLink/ParentInviteBox'), 0)

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/CalendarLink/InviteParentviaButtonClose'))

WebUI.closeBrowser()

