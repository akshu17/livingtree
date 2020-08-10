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
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_TEACHER)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.delay(30)

WebUI.click(findTestObject('LivingTree/ConversationPage/ShareMessageInputBox'))

WebUI.setText(findTestObject('LivingTree/SchedulePost/typetextBox'), 'test')

WebUI.delay(2)

WebUI.setText(findTestObject('LivingTree/SchedulePost/TypeMessageBox'), 'test')

WebUI.delay(1)

WebUI.scrollToElement(findTestObject('LivingTree/SchedulePost/IwanttosharethihwithBox'), 1)

WebUI.click(findTestObject('LivingTree/SchedulePost/IwanttosharethihwithBox'))

WebUI.delay(1)

WebUI.verifyElementClickable(findTestObject('LivingTreeProduction/MyNetworkPage/ScienceGroup'))

WebUI.click(findTestObject('LivingTreeProduction/MyNetworkPage/ScienceGroup'))

WebUI.scrollToElement(findTestObject('LivingTree/SchedulePost/StudentsGroup'), 0)

WebUI.click(findTestObject('LivingTree/SchedulePost/StudentsGroup'))

WebUI.check(findTestObject('LivingTree/SchedulePost/CheckSchedulePostBox'))

WebUI.delay(3)

WebUI.mouseOver(findTestObject('LivingTree/SchedulePost/ScheduledTime'))

WebUI.scrollToElement(findTestObject('LivingTree/SchedulePost/ScheduledTime'), 7)

WebUI.click(findTestObject('LivingTree/SchedulePost/ScheduledTime'))

WebUI.delay(2)

WebUI.mouseOver(findTestObject('LivingTree/SchedulePost/ScheduleDateBox'))

WebUI.click(findTestObject('LivingTree/SchedulePost/ScheduleDateBox'))

WebUI.click(findTestObject('LivingTree/SchedulePost/SavePostButton'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/SchedulePost/ScheduledEditPencil'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/SchedulePost/SheduleUpdatePostButton'))

WebUI.mouseOver(findTestObject('LivingTree/SchedulePost/ConversationsNotification'))

WebUI.closeBrowser()

