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

WebUI.delay(5)

WebUI.click(findTestObject('LivingTree/PollPostStyle/ConversationSearchBox'))

WebUI.setText(findTestObject('LivingTree/PollPostStyle/CreatePostSubjectBox'), 'test')

WebUI.delay(3)

WebUI.click(findTestObject('LivingTree/PollPostStyle/AddPollButton'))

WebUI.setText(findTestObject('LivingTree/PollPostStyle/AnswerBoxOne'), 'test')

WebUI.setText(findTestObject('LivingTree/PollPostStyle/AnswerBoxTwo'), 'test')

WebUI.click(findTestObject('LivingTree/PollPostStyle/IWantToShareWIth'))

WebUI.click(findTestObject('LivingTree/PollPostStyle/RymsGroup'))

WebUI.click(findTestObject('Page_LivingTree teacher taa/StudentsGroup'))

WebUI.click(findTestObject(null))

