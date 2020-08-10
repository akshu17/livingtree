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

WebUI.delay(7)

WebUI.click(findTestObject('LivingTree/ConversationPage/ShareMessageInputBox'))

WebUI.delay(1)

WebUI.waitForElementPresent(findTestObject('LivingTree/SchedulePost/CheckSchedulePostBox'), 0)

WebUI.click(findTestObject('LivingTree/SchedulePost/CheckSchedulePostBox'))

WebUI.uncheck(findTestObject('LivingTree/SchedulePost/CheckSchedulePostBox'))

WebUI.check(findTestObject('LivingTree/SchedulePost/ScheduleBox'))

WebUI.uncheck(findTestObject('LivingTree/SchedulePost/ScheduleBox'))

WebUI.check(findTestObject('LivingTree/SchedulePost/SchedulePostText'))

WebUI.click(findTestObject('LivingTree/SchedulePost/ScheduleDateIcon'))

WebUI.click(findTestObject('LivingTree/SchedulePost/ScheduledTime'))

WebUI.click(findTestObject('LivingTree/ConversationPage/CreatePostCloseButton'))

WebUI.closeBrowser()

