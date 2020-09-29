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

WebUI.click(findTestObject('LivingTree/MyNetworkPage/MyNetworkPageIcon'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/SelectGroupBox'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/IntercomButton'), 0)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/FamilyEditPencilButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkFamilyBox'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/FamilyButtonCancel'), 0)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/FamilyButtonCancel'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/CreateMirrorGroupButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MirrorGroupBox'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/div_Close Create Mirror Group QA Family'), 1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/MirrorGroupButtonClose'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/RymsGroup'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkEditPencilRymsGroup'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkRymsBox'), 1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/RymsButtonCancel'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/span_Assistants and other Administrators_hoverButton'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkInviteParentButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkInvitePopUpParentBox'), 1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/parent'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MynetworkParentBox'), 2)

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/Page_LivingTree teacher taa/button_Close'))

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkInviteStudentButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkStudentPopUpBox'), 1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/Student'))

WebUI.waitForElementPresent(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkStudentBox'), 1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/Page_LivingTree Ryms/button_Close'))

WebUI.closeBrowser()

